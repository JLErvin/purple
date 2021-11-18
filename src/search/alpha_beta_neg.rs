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

pub struct AlphaBetaNeg {
    gen: MoveGenerator,
    stats: Stats,
    zobrist: ZobristTable,
    table: TranspositionTable,
}

impl Searcher for AlphaBetaNeg {
    fn new() -> Self {
        let gen = MoveGenerator::new();
        let stats = Stats::new();
        let zobrist = crate::table::zobrist::ZobristTable::init();
        let table = TranspositionTable::new_mb(50);
        AlphaBetaNeg {
            gen,
            stats,
            zobrist,
            table,
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

impl AlphaBetaNeg {
    fn alpha_beta(
        &mut self,
        pos: &mut BoardState,
        mut alpha: isize,
        beta: isize,
        depth: u8,
    ) -> EvaledMove {
        // First, see if we can get a TT hit
        let hash = self.zobrist.hash(pos);
        let entry = self.table.get(hash, depth as usize);
        if let Some(e) = entry {
            let is_bound_ok = match e.bound {
                Bound::Lower => e.best_move.eval >= beta,
                Bound::Upper => e.best_move.eval <= alpha,
                Bound::Exact => true 
            };
            if e.depth >= depth && is_bound_ok && e.hash == hash {
                return e.best_move
            } 
        };

        // Otherwise, search normally
        if depth == 0 {
            self.stats.count_node();
            let eval = EvaledMove::null(eval(pos));

            let bound = if eval.eval >= beta {
                Bound::Lower
            } else if eval.eval > alpha {
                Bound::Exact
            } else {
                Bound::Upper
            };

            let hash = self.zobrist.hash(pos);
            let entry = Entry {
                best_move: eval,
                depth,
                bound,
                hash,
            };

            self.table.save(hash, entry, depth as usize);

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
                    let hash = self.zobrist.hash(&mut new_pos);
                    let entry = Entry {
                        best_move: *mv,
                        depth,
                        bound: Bound::Lower,
                        hash,
                    };

                    self.table.save(hash, entry, depth as usize);
                    return *mv;
                }
                best_move = *mv;
            }
        }

        let bound = if best_move.eval > prev_alpha { Bound::Exact } else { Bound::Upper };
        //let bound = Bound::Upper;
        let hash = self.zobrist.hash(pos);
        let entry = Entry {
            hash,
            best_move,
            depth,
            bound
        };
        self.table.save(hash, entry, depth as usize);

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
}

pub fn eval(pos: &BoardState) -> isize {
    material_eval(pos) + mobility_eval(pos) + pawn_eval(pos)
}

const PAWN_VALUE: isize = 100;
const ROOK_VALUE: isize = 500;
const KNIGHT_VALUE: isize = 300;
const BISHOP_VALUE: isize = 300;
const KING_VALUE: isize = 350;
const QUEEN_VALUE: isize = 800;

pub const INF: isize = 32_001;
pub const NEG_INF: isize = -32_001;

#[inline]
fn material_eval(pos: &BoardState) -> isize {
    let pawn_eval = piece_difference(pos, PieceType::Pawn) * PAWN_VALUE;
    let rook_eval = piece_difference(pos, PieceType::Rook) * ROOK_VALUE;
    let knight_eval = piece_difference(pos, PieceType::Knight) * KNIGHT_VALUE;
    let bishop_eval = piece_difference(pos, PieceType::Bishop) * BISHOP_VALUE;
    let queen_eval = piece_difference(pos, PieceType::Queen) * QUEEN_VALUE;
    let king_eval = piece_difference(pos, PieceType::King) * KING_VALUE;

    pawn_eval + rook_eval + knight_eval + bishop_eval + queen_eval + king_eval
}

#[inline]
fn piece_difference(pos: &BoardState, piece: PieceType) -> isize {
    num_pieces(pos, pos.active_player(), piece) - num_pieces(pos, !pos.active_player(), piece)
}

#[inline]
fn num_pieces(pos: &BoardState, color: Color, piece: PieceType) -> isize {
    pos.bb(color, piece).iter().count() as isize
}

#[inline]
fn mobility_eval(pos: &BoardState) -> isize {
    0
}

#[inline]
fn pawn_eval(pos: &BoardState) -> isize {
    0
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
