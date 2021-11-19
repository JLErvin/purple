use super::{eval::MATE_VALUE, search::Searcher};
use crate::{
    board_state::board::BoardState,
    common::{
        bitboard::PieceItr,
        chess_move::Move,
        eval_move::EvaledMove,
        lookup::Lookup,
        piece::{Color, PieceType},
        stats::Stats,
    },
    magic::random::{GenerationScheme, MagicRandomizer},
    move_gen::{
        generator::MoveGenerator,
        util::{is_attacked, king_square},
    },
    table::{
        transposition::{Bound, Entry, TranspositionTable},
        zobrist::ZobristTable,
    },
};
use itertools::Itertools;
use std::cmp::{max, min};
use crate::search::eval::{eval, INF, NEG_INF};

pub struct Settings {
    use_table: bool,
}

pub struct AlphaBetaNeg {
    gen: MoveGenerator,
    stats: Stats,
    zobrist: ZobristTable,
    table: TranspositionTable,
    settings: Settings
}

impl Searcher for AlphaBetaNeg {
    fn new() -> Self {
        let gen = MoveGenerator::new();
        let stats = Stats::new();
        let zobrist = crate::table::zobrist::ZobristTable::init();
        let table = TranspositionTable::new_mb(50);
        let settings = Settings { use_table: true };
        AlphaBetaNeg {
            gen,
            stats,
            zobrist,
            table,
            settings
        }
    }

    fn stats(&self) -> &Stats {
        &self.stats
    }

    fn best_move(&mut self, pos: &mut BoardState) -> EvaledMove {
        self.stats.reset();
        self.best_move_depth(pos, 5)
    }

    fn best_move_depth(&mut self, pos: &mut BoardState, depth: usize) -> EvaledMove {
        self.stats.reset();
        self.alpha_beta(pos, NEG_INF, INF, depth as u8)
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

/// Returns the proper Bound when evaluating a leaf node in alpha beta
fn leaf_bound(best_move: EvaledMove, alpha: isize, beta: isize) -> Bound {
    if best_move.eval >= beta {
        Bound::Lower
    } else if best_move.eval > alpha {
        Bound::Exact
    } else {
        Bound::Upper
    }
}

impl AlphaBetaNeg {
    fn alpha_beta(
        &mut self,
        pos: &mut BoardState,
        mut alpha: isize,
        beta: isize,
        depth: u8,
    ) -> EvaledMove {
        if let Some(e) = self.table_fetch(pos, alpha, beta, depth) {
            return e;
        }

        if depth == 0 {
            self.stats.count_node();
            //let eval = self.q_search(pos, alpha, beta, 5);
            let eval = EvaledMove::null(self.q_search(pos, alpha, beta, 5));
            let bound = leaf_bound(eval, alpha, beta);
            self.save(pos, eval, bound, depth);

            return eval;
        }

        let mut moves = evaled_moves(self.gen.all_moves(pos));

        if moves.is_empty() {
            self.stats.count_node();
            return self.no_move_eval(pos, depth as usize);
        }

        let mut prev_alpha = alpha;
        let mut best_move = EvaledMove::null(alpha);
        for mv in moves.iter_mut() {
            let mut new_pos = pos.clone_with_move(mv.mv);
            mv.eval = -self.alpha_beta(&mut new_pos, -beta, -alpha, depth - 1).eval;
            if mv.eval > alpha {
                alpha = mv.eval;
                if alpha >= beta {
                    self.save(pos, *mv, Bound::Lower, depth);
                    return *mv;
                }
                best_move = *mv;
            }
        }

        let bound = if best_move.eval > prev_alpha {
            Bound::Exact
        } else {
            Bound::Upper
        };
        self.save(pos, best_move, bound, depth);

        best_move
    }

    fn q_search(&mut self, pos: &mut BoardState, mut alpha: isize, beta: isize, depth: usize) -> isize {
        if depth == 0 {
            return eval(pos)
        }

        let mut moves = if is_attacked(pos, king_square(pos), &self.gen.lookup) {
            self.gen.all_moves(pos)
        } else {
            self.gen.all_moves(pos).into_iter().filter(|mv| mv.is_capture()).collect()
        };

        if moves.is_empty() {
            self.stats.count_node();
            return self.no_move_eval(pos, depth).eval;
        }

        let mut best_move = 0;
        for mv in moves.iter_mut() {
            let mut new_pos = pos.clone_with_move(*mv);
            let eval = -self.q_search(&mut new_pos, -beta, -alpha, depth - 1);
            if eval > alpha {
                alpha = eval;
                best_move = alpha;
                if alpha >= beta {
                    return eval;
                }
            }
        }
        best_move
    }

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
    fn table_fetch(
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
        if entry.is_none() {
            return None;
        };
        let entry = entry.unwrap();
        return if entry.depth >= depth && is_bound_ok(&entry, alpha, beta) {
            Some(entry.best_move)
        } else {
            None
        };
    }

    /// Saves the given entry in the transposition table.
    fn save(&mut self, pos: &mut BoardState, best_move: EvaledMove, bound: Bound, depth: u8) {
        if !self.settings.use_table {
            return;
        }

        let hash = self.zobrist.hash(pos);
        let entry = Entry {
            best_move,
            depth,
            bound,
            hash,
        };
        self.table.save(hash, entry);
    }

    /// Set whether or not the searcher should use a transposition table to lookup previous evaluations.
    fn use_table(&mut self, setting: bool) {
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

mod test {
    use super::*;
    use crate::board_state::fen::parse_fen;
    use crate::move_gen::generator::debug_print;

    #[test]
    fn finds_mate_in_one_as_white() {
        let mut pos = parse_fen(&"k7/8/2K5/8/8/8/8/1Q6 w - - 0 1".to_string()).unwrap();
        let mut searcher: AlphaBetaNeg = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 49)
    }

    #[test]
    fn finds_mate_in_one_as_black() {
        let mut pos = parse_fen(&"K7/8/2k5/8/8/8/8/1q6 b - - 0 1".to_string()).unwrap();
        let mut searcher: AlphaBetaNeg = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 49)
    }

    #[test]
    fn best_move_random_1() {
        let mut pos =
            parse_fen(&"r2qkbnr/ppp2ppp/2np4/8/8/PPPpPbP1/7P/RNBQKBNR w KQkq - 0 8".to_string())
                .unwrap();
        let mut searcher: AlphaBetaNeg = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 21)
    }

    #[test]
    fn best_move_random_2() {
        let mut pos =
            parse_fen(&"rnbqkbnr/7p/pppPpBp1/8/8/3P4/PPP2PPP/R2QKBNR b - - 0 1".to_string())
                .unwrap();
        let mut searcher: AlphaBetaNeg = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 45)
    }

    #[test]
    fn best_move_random_3() {
        let mut pos =
            parse_fen(&"r2qkbnr/ppp2ppp/2np4/8/8/PPPpPbP1/7P/RNBQKBNR b KQkq - 0 8".to_string())
                .unwrap();
        let mut searcher: AlphaBetaNeg = Searcher::new();
        let mv = searcher.best_move(&mut pos);
        debug_print(&pos);
        assert_eq!(mv.mv.to, 3)
    }
}
