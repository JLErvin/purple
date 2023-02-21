use std::time::Instant;

use itertools::Itertools;
use rayon::slice::ParallelSliceMut;

use super::eval::MATE_VALUE;
use super::search::Searcher;
use crate::board::BoardState;
use crate::chess_move::{self, EvaledMove, Move, MoveType};
use crate::move_gen::{is_attacked, king_square, MoveGenerator};
use crate::piece::PieceType;
use crate::search::eval::{eval, INF, NEG_INF};
use crate::search::stats::Stats;
use crate::table::{Bound, Entry, TranspositionTable, ZobristTable};

pub struct Settings {
    use_table: bool,
    move_time: Option<u64>,
}

pub struct AlphaBeta {
    pub gen: MoveGenerator,
    stats: Stats,
    zobrist: ZobristTable,
    table: TranspositionTable,
    settings: Settings,
    start_time: Instant,
}

impl Searcher for AlphaBeta {
    fn new() -> Self {
        let gen = MoveGenerator::new();
        let stats = Stats::new();
        let zobrist = ZobristTable::init();
        let table = TranspositionTable::new_mb(50);
        let settings = Settings {
            use_table: true,
            move_time: None,
        };
        let start_time = Instant::now();
        AlphaBeta {
            gen,
            stats,
            zobrist,
            table,
            settings,
            start_time,
        }
    }

    fn stats(&self) -> &Stats {
        &self.stats
    }

    fn best_move(&mut self, pos: &mut BoardState) -> EvaledMove {
        self.stats.reset();
        self.best_move_depth(pos, 6)
    }

    /// Performs an iterative deepening search until the specified depth and returns the best move
    fn best_move_depth(&mut self, pos: &mut BoardState, deth: usize) -> EvaledMove {
        self.start_time = Instant::now();

        let mut best_move: EvaledMove = EvaledMove::null(0);
        let mut i = 0;
        //for i in 0..=depth {
        loop {
            let now = Instant::now();
            let elapsed = now.duration_since(self.start_time).as_secs();
            if elapsed > self.settings.move_time.unwrap() as u64 {
                break;
            }

            let next = self.alpha_beta(pos, NEG_INF, INF, i as u8);
            if next.is_none() {
                break;
            }
            best_move = next.unwrap();
            i += 1;
            println!("depth: {}, nodes: {}", i, self.stats.nodes);
            self.stats.reset();
        }
        //let mut pv = Vec::new();
        //pv = self.table.pv(pos, &self.zobrist);
        //println!("PV: {:?}", pv);

        best_move
    }

    fn move_time(&mut self, seconds: u64) {
        self.settings.move_time = Some(seconds);
    }
}

/// Given an entry to save and values for alpha/beta in a negamax implementation, returns whether
/// or not the given entry can be used for those values of alpha and beta in a TT lookup
fn is_bound_ok(entry: &Entry, alpha: isize, beta: isize) -> bool {
    match entry.bound {
        Bound::Lower => entry.best_move.eval >= beta,
        Bound::Upper => entry.best_move.eval <= alpha,
        Bound::Exact => true,
    }
}

/// Given an evaluated move and values and alpha and beta, return the bound that should be stored in a
/// transposition table assuming the given move is at a leaf node.
fn leaf_bound(best_move: EvaledMove, alpha: isize, beta: isize) -> Bound {
    if best_move.eval >= beta {
        Bound::Lower
    } else if best_move.eval > alpha {
        Bound::Exact
    } else {
        Bound::Upper
    }
}

impl AlphaBeta {
    fn alpha_beta(
        &mut self,
        pos: &mut BoardState,
        mut alpha: isize,
        beta: isize,
        depth: u8,
    ) -> Option<EvaledMove> {
        // If time has expired, ignore this search request
        let now = Instant::now();
        let elapsed = now.duration_since(self.start_time).as_secs();
        if elapsed > self.settings.move_time.unwrap() as u64 {
            return None;
        }

        if let Some(e) = self.table_fetch(pos, alpha, beta, depth) {
            self.stats.count_node();
            return Some(e);
        }

        let prev_alpha = alpha;
        let mut best_move = EvaledMove::null(alpha);
        let mut moves = Vec::<EvaledMove>::new();

        let hash = self.zobrist.hash(pos);
        if let Some(e) = self.table.get(hash) {
            if e.hash == hash && e.depth >= depth && is_bound_ok(&e, alpha, beta) {
                self.stats.count_node();
                return Some(e.best_move);
            }

            if e.hash == hash && e.best_move.mv.kind != MoveType::Null {
                moves.push(e.best_move);
            }
        }

        if depth == 0 {
            self.stats.count_node();
            let s = EvaledMove::null(self.q_search(pos, alpha, beta, 5));
            let bound = leaf_bound(s, alpha, beta);
            self.save(pos, s, bound, depth);
            self.stats.count_node();
            return Some(s);
        }

        let mut gen = evaled_moves(self.gen.all_moves(pos));
        sort_moves(&mut gen, pos);
        moves.append(&mut gen);
        //let mut new = sort_moves(&mut gen, pos);
        //moves.append(&mut new);

        if moves.is_empty() {
            self.stats.count_node();
            return Some(self.no_move_eval(pos, depth as usize));
        }

        for mv in &mut moves {
            let mut new_pos = pos.clone_with_move(mv.mv);
            let next = self.alpha_beta(&mut new_pos, -beta, -alpha, depth - 1);
            if next.is_none() {
                return next;
            }

            mv.eval = -next.unwrap().eval;
            if mv.eval > alpha {
                alpha = mv.eval;
                best_move = *mv;
                if alpha >= beta {
                    self.save(pos, *mv, Bound::Lower, depth);
                    self.stats.count_node();
                    return Some(best_move);
                }
            }
        }

        let bound = if best_move.eval > prev_alpha {
            Bound::Exact
        } else {
            Bound::Upper
        };
        self.save(pos, best_move, bound, depth);

        self.stats.count_node();
        Some(best_move)
    }

    /// Perform a Quiescence search, which evaluates up to a certain provided maximum depth
    /// or until a position reaches a "quiet" state (i.e., one in which there are no captures).
    fn q_search(
        &mut self,
        pos: &mut BoardState,
        mut alpha: isize,
        beta: isize,
        depth: usize,
    ) -> isize {
        let eval = eval(pos);
        let now = Instant::now();
        let elapsed = now.duration_since(self.start_time).as_secs();
        if elapsed > self.settings.move_time.unwrap() as u64 {
            return eval;
        }

        if depth == 0 {
            return eval;
        }

        if eval >= beta {
            return beta;
        } else if eval > alpha {
            alpha = eval;
        };

        let is_attacked = is_attacked(pos, king_square(pos), &self.gen.lookup);

        let mut moves = if is_attacked {
            self.gen.all_moves(pos)
        } else {
            self.gen
                .all_moves(pos)
                .into_iter()
                .filter(chess_move::Move::is_capture)
                .collect()
        };
        //sort_moves_2(&mut moves, pos);

        if moves.is_empty() && is_attacked {
            return self.no_move_eval(pos, depth).eval;
        }

        for mv in moves.iter_mut() {
            let mut new_pos = pos.clone_with_move(*mv);
            let eval = -self.q_search(&mut new_pos, -beta, -alpha, depth - 1);
            if eval >= beta {
                return beta;
            }

            if eval > alpha {
                alpha = eval;
            }
        }
        alpha
    }

    /// Return an evaluation of the given position, at the given depth, assuming there are no valid
    /// moves in the position. The returned value is either 0 (a draw), or is less than being mated
    /// by the moving player (i.e., a value of -MATE_VALUE).
    fn no_move_eval(&self, pos: &BoardState, depth: usize) -> EvaledMove {
        let is_in_check = is_attacked(pos, king_square(pos), &self.gen.lookup);

        if is_in_check {
            EvaledMove::null(-MATE_VALUE - depth as isize)
        } else {
            EvaledMove::null(0)
        }
    }

    /// Given a position, alpha/beta, and a depth from the bottom of the tree, attempts to fetch the
    /// evaluated move from the transposition table. Only entries with valid bounds and depths will
    /// be returned.
    pub fn table_fetch(
        &self,
        pos: &mut BoardState,
        alpha: isize,
        beta: isize,
        depth: u8,
    ) -> Option<EvaledMove> {
        if !self.settings.use_table {
            return None;
        }

        let hash = self.zobrist.hash(pos);
        let entry = self.table.get(hash);
        entry?;
        let entry = entry.unwrap();
        if entry.hash == hash && entry.depth >= depth && is_bound_ok(&entry, alpha, beta) {
            Some(entry.best_move)
        } else {
            None
        }
    }

    /// Saves the given entry in the transposition table.
    fn save(&mut self, pos: &mut BoardState, best_move: EvaledMove, bound: Bound, depth: u8) {
        if !self.settings.use_table {
            return;
        }

        let hash = self.zobrist.hash(pos);
        //let fen = debug_print(pos);
        let entry = Entry {
            best_move,
            depth,
            bound,
            hash,
            //fen
        };
        self.table.save(hash, entry);
    }

    /// Set whether or not the searcher should use a transposition table to lookup previous evaluations.
    #[allow(dead_code)]
    pub fn use_table(&mut self, setting: bool) {
        self.settings.use_table = setting;
    }
}

#[inline]
fn evaled_moves(moves: Vec<Move>) -> Vec<EvaledMove> {
    moves
        .iter()
        .map(|mv| EvaledMove { mv: *mv, eval: 0 })
        .collect_vec()
}

pub const MVV_LVA: [[isize; 6]; 6] = [
    [0, 0, 0, 0, 0, 0],       // victim K, attacker K, Q, R, B, N, P, None
    [50, 51, 52, 53, 54, 55], // victim Q, attacker K, Q, R, B, N, P, None
    [40, 41, 42, 43, 44, 45], // victim R, attacker K, Q, R, B, N, P, None
    [30, 31, 32, 33, 34, 35], // victim B, attacker K, Q, R, B, N, P, None
    [20, 21, 22, 23, 24, 25], // victim N, attacker K, Q, R, B, N, P, None
    [10, 11, 12, 13, 14, 15], // victim P, attacker K, Q, R, B, N, P, None
];

/*
fn sort_moves_2(moves: &mut Vec<Move>, pos: &BoardState) {
    moves.par_sort_unstable_by_key(|mv| {
        let maybe_capturing_piece = pos.type_on(mv.from).unwrap();
        if mv.is_en_passant_capture() {
            return 0;
        }

        if mv.is_capture() {
            let captured_piece = pos.type_on(mv.to).unwrap();
            return MVV_LVA[captured_piece.idx()][maybe_capturing_piece.idx()];
        }
        0
    });
}
*/

fn sort_moves_2(moves: &mut Vec<Move>, pos: &BoardState) {
    moves.par_sort_unstable_by_key(|mv| {
        let maybe_capturing_piece = pos.type_on(mv.from).unwrap();
        if mv.is_en_passant_capture() {
            return 0;
        }

        if mv.is_capture() {
            let captured_piece = pos.type_on(mv.to).unwrap();
            return -MVV_LVA[captured_piece.idx()][maybe_capturing_piece.idx()];
        }

        0
    });
}

fn sort_moves(moves: &mut Vec<EvaledMove>, pos: &BoardState) {
    moves.par_sort_unstable_by_key(|mv| {
        if !mv.mv.is_capture() {
            return 0;
        }

        if mv.mv.kind == MoveType::EnPassantCapture {
            return 1;
        }

        let capturing_piece = pos.type_on(mv.mv.from).unwrap();
        let captured_piece = pos.type_on(mv.mv.to).unwrap();

        // If we are capturing a more valuable piece, return a very negative number
        return if captured_piece.value() > capturing_piece.value() {
            capturing_piece.value() - captured_piece.value() - 100
        } else {
           captured_piece.value() - capturing_piece.value() - 50
        }
    });
    /*
    moves.sort_by_cached_key(|mv| {
        let maybe_capturing_piece = pos.type_on(mv.mv.from).unwrap();
        if mv.mv.is_en_passant_capture() {
            return 0;
        }

        if mv.mv.is_capture() {
            let captured_piece = pos.type_on(mv.mv.to).unwrap();
            return MVV_LVA[captured_piece.idx()][maybe_capturing_piece.idx()] - 100;
        }

        0
    });
    */
}

pub struct WeightedMove {
    pub mv: EvaledMove,
    pub weight: isize,
}

#[inline]
fn weighted_moves(moves: &[EvaledMove], pos: &BoardState) -> Vec<WeightedMove> {
    moves
        .iter()
        .map(|mv: &EvaledMove| {
            let weight = match mv.mv.kind {
                MoveType::Capture => {
                    let maybe_capturing_piece = pos.type_on(mv.mv.from).unwrap();
                    let captured_piece = pos.type_on(mv.mv.to).unwrap();

                    if mv.mv.is_en_passant_capture() {
                        0
                    } else {
                        MVV_LVA[captured_piece.idx()][maybe_capturing_piece.idx()]
                    }
                }
                _ => 0,
            };

            WeightedMove { mv: *mv, weight }
        })
        .collect_vec()
}

#[cfg(test)]
mod test {
    use super::{evaled_moves, sort_moves};
    use crate::chess_move::MoveType;
    use crate::fen::parse_fen;
    use crate::search::alpha_beta::AlphaBeta;
    use crate::search::search::Searcher;
    use crate::square::SquareIndex::C5;

    #[test]
    fn finds_mate_in_one_as_white() {
        let mut pos = parse_fen(&"k7/8/2K5/8/8/8/8/1Q6 w - - 0 1".to_string()).unwrap();
        let mut searcher: AlphaBeta = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 49)
    }

    #[test]
    fn finds_mate_in_one_as_black() {
        let mut pos = parse_fen(&"K7/8/2k5/8/8/8/8/1q6 b - - 0 1".to_string()).unwrap();
        let mut searcher: AlphaBeta = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 49)
    }

    #[test]
    fn best_move_random_1() {
        let mut pos =
            parse_fen(&"r2qkbnr/ppp2ppp/2np4/8/8/PPPpPbP1/7P/RNBQKBNR w KQkq - 0 8".to_string())
                .unwrap();
        let mut searcher: AlphaBeta = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 21)
    }

    #[test]
    fn best_move_random_2() {
        let mut pos =
            parse_fen(&"rnbqkbnr/7p/pppPpBp1/8/8/3P4/PPP2PPP/R2QKBNR b - - 0 1".to_string())
                .unwrap();
        let mut searcher: AlphaBeta = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 45)
    }

    #[test]
    fn best_move_random_3() {
        let mut pos =
            parse_fen(&"r2qkbnr/ppp2ppp/2np4/8/8/PPPpPbP1/7P/RNBQKBNR b KQkq - 0 8".to_string())
                .unwrap();
        let mut searcher: AlphaBeta = Searcher::new();
        let mv = searcher.best_move(&mut pos);
        assert_eq!(mv.mv.to, 3)
    }

    #[test]
    fn avoids_horizon() {
        let mut pos = parse_fen(&"7k/8/r7/r7/8/8/p1RR3K/8 w - - 0 1".to_string()).unwrap();
        let mut searcher: AlphaBeta = Searcher::new();
        let mv = searcher.best_move_depth(&mut pos, 3);
        assert_ne!(mv.mv.to, 8)
    }

    #[test]
    fn doesnt_blunder() {
        let mut pos = parse_fen(&"2Q5/1K6/5k2/8/3bB3/8/8/8 b - - 0 72".to_string()).unwrap();
        let mut searcher: AlphaBeta = Searcher::new();
        let mv = searcher.best_move_depth(&mut pos, 5);
        assert_ne!(mv.mv.to, 8)
    }

    #[test]
    #[ignore]
    fn doesnt_blunder_2() {
        let mut searcher: AlphaBeta = Searcher::new();
        let mut pos =
            parse_fen(&"rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1".to_string())
                .unwrap();
        let _mv = searcher.best_move_depth(&mut pos, 7);
        let mut pos =
            parse_fen(&"rnbqkbnr/1ppppppp/p7/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 1 2".to_string())
                .unwrap();
        let _mv = searcher.best_move_depth(&mut pos, 7);
        let mut pos = parse_fen(
            &"rnbqkbnr/1ppppppp/8/p7/3P4/1PN5/P1P1PPPP/R1BQKBNR b KQkq - 0 3".to_string(),
        )
        .unwrap();
        let _mv = searcher.best_move_depth(&mut pos, 7);

        let mut pos = parse_fen(
            &"rnbqkbnr/2pppppp/1p6/p7/3P4/1PN5/PBP1PPPP/R2QKBNR b KQkq - 1 4".to_string(),
        )
        .unwrap();
        let _mv = searcher.best_move_depth(&mut pos, 7);
    }

    #[test]
    fn doesnt_blunder_3() {
        let mut pos = parse_fen(
            &"rnbqk1nr/3p3p/2p1pppb/8/Pp1PPP2/3B2P1/PBPQ3P/1NKR3R b kq - 0 14".to_string(),
        )
        .unwrap();
        let mut searcher: AlphaBeta = Searcher::new();
        let mv = searcher.best_move_depth(&mut pos, 5);
        assert_ne!(mv.mv.to, 17)
    }

    #[test]
    fn sorts_captures_over_non_captures() {
        // Any piece can capture the opposing queen
        let pos = parse_fen(&"7k/8/8/2q2Q2/1P6/3N4/5B2/K1R5 w - - 0 1".to_string()).unwrap();

        let searcher: AlphaBeta = Searcher::new();
        let mut moves = evaled_moves(searcher.gen.all_moves(&pos));
        println!("{:?}", moves);
        println!();
        sort_moves(&mut moves, &pos);
        println!("{:?}", moves);

        let top_move = moves[0];
        assert_eq!(top_move.mv.kind, MoveType::Capture);
    }

    #[test]
    fn sorts_better_captures_over_other_captures() {
        // Rook can take either pawn or queen
        let pos = parse_fen(&"4k3/8/8/2p5/8/2Qq4/8/K7 w - - 0 1".to_string()).unwrap();

        let searcher: AlphaBeta = Searcher::new();
        let mut moves = evaled_moves(searcher.gen.all_moves(&pos));
        println!("{:?}", moves);
        println!();
        sort_moves(&mut moves, &pos);
        println!("{:?}", moves);

        let top_move = moves[0];
        assert_eq!(top_move.mv.kind, MoveType::Capture);
        assert_eq!(top_move.mv.to, C5 as u8);
    }
}
