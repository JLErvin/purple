use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::{max, min};

use super::{
    eval::{eval, no_move_eval, INF, MATE_VALUE, NEG_INF},
    search::Searcher,
};
use crate::{
    board_state::board::BoardState,
    common::{
        bitboard::PieceItr,
        chess_move::Move,
        eval_move::EvaledMove,
        piece::{Color, PieceType},
        stats::Stats,
    },
    move_gen::{
        generator::MoveGenerator,
        util::{is_attacked, king_square},
    },
};

const PAWN_VALUE: isize = 100;
const ROOK_VALUE: isize = 500;
const KNIGHT_VALUE: isize = 300;
const BISHOP_VALUE: isize = 300;
const KING_VALUE: isize = 350;
const QUEEN_VALUE: isize = 800;

pub struct ParallelMinimaxSearcher {
    gen: MoveGenerator,
    stats: Stats,
}

impl Searcher for ParallelMinimaxSearcher {
    fn new() -> Self {
        let gen = MoveGenerator::new();
        let stats = Stats::new();
        ParallelMinimaxSearcher { gen, stats }
    }

    fn stats(&self) -> &Stats {
        &self.stats
    }

    fn best_move(&mut self, pos: &mut BoardState) -> EvaledMove {
        self.stats.reset();
        ParallelMinimaxSearcher::minimax(pos, &self.gen, 5)
        //self.minimax(pos, 5)
    }

    fn best_move_depth(&mut self, pos: &mut BoardState, depth: usize) -> EvaledMove {
        self.stats.reset();
        ParallelMinimaxSearcher::minimax(pos, &self.gen, depth)
    }
}

impl ParallelMinimaxSearcher {
    fn minimax(pos: &mut BoardState, gen: &MoveGenerator, depth: usize) -> EvaledMove {
        if depth == 0 {
            //self.stats.count_node();
            return EvaledMove::null(eval(pos));
        }

        let moves = evaled_moves(gen.all_moves(pos));
        if moves.is_empty() {
            //self.stats.count_node();
            return no_move_eval(pos, depth);
        }

        let moves = moves.into_par_iter().map(|mut mv: EvaledMove| {
            let mut new_pos = pos.clone_with_move(mv.mv);
            mv.eval = ParallelMinimaxSearcher::minimax(&mut new_pos, gen, depth - 1).eval;
            mv
        });

        let best_move = if pos.active_player() == Color::White {
            moves.max().unwrap()
        } else {
            moves.min().unwrap()
        };

        /*
        let best_move = moves.into_iter()
        .map(|mut mv: EvaledMove| {
            let mut new_pos = pos.clone_with_move(mv.mv);
            mv.eval = -self.minimax(&mut new_pos, depth - 1).eval;
            mv
        })
        .max().unwrap_or_else(|| match in_check(pos, &self.gen) {
            true => EvaledMove::null(-MATE_VALUE - depth as isize),
            false => EvaledMove::null(0)
        });
        */

        best_move
    }
}

fn in_check(pos: &BoardState, gen: &MoveGenerator) -> bool {
    let king_square = king_square(pos);
    is_attacked(pos, king_square, &gen.lookup)
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
        let mut searcher: ParallelMinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        //println!("{}", mv.eval);
        assert_eq!(mv.to, 49)
    }

    #[test]
    fn finds_mate_in_one_as_black() {
        let mut pos = parse_fen(&"K7/8/2k5/8/8/8/8/1q6 b - - 0 1".to_string()).unwrap();
        let mut searcher: ParallelMinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 49)
    }

    #[test]
    fn best_move_random_1() {
        let mut pos =
            parse_fen(&"r2qkbnr/ppp2ppp/2np4/8/8/PPPpPbP1/7P/RNBQKBNR w KQkq - 0 8".to_string())
                .unwrap();
        let mut searcher: ParallelMinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 21)
    }

    #[test]
    fn best_move_random_2() {
        let mut pos =
            parse_fen(&"rnbqkbnr/7p/pppPpBp1/8/8/3P4/PPP2PPP/R2QKBNR b - - 0 1".to_string())
                .unwrap();
        let mut searcher: ParallelMinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 45)
    }

    #[test]
    fn best_move_random_3() {
        let mut pos =
            parse_fen(&"r2qkbnr/ppp2ppp/2np4/8/8/PPPpPbP1/7P/RNBQKBNR b KQkq - 0 8".to_string())
                .unwrap();
        let mut searcher: ParallelMinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos);
        debug_print(&pos);
        println!("{}", mv.eval);
        assert_eq!(mv.mv.to, 3)
    }
    #[test]
    fn best_move_random_4() {
        let mut pos = parse_fen(
            &"rnbqkbnr/1p1ppppp/2p5/8/p2PP2P/2N2N2/PPP2PP1/R1BQKB1R b KQkq - 0 5".to_string(),
        )
        .unwrap();
        let mut searcher: ParallelMinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos);
        println!("{}", mv.eval);
        println!("to: {}", mv.mv.to);
        println!("from: {}", mv.mv.from);
    }
}
