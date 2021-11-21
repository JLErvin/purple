use crate::common::eval_move::EvaledMove;
use crate::common::piece::Color;
use crate::common::stats::Stats;
use crate::move_gen::generator::MoveGenerator;
use crate::move_gen::util::{is_attacked, king_square};
use crate::search::eval::{eval, INF, MATE_VALUE};
use crate::{BoardState, Move, Searcher};
use itertools::Itertools;
use std::cmp::{max, min};

pub struct MinimaxSearcher {
    gen: MoveGenerator,
    stats: Stats,
}

impl Searcher for MinimaxSearcher {
    fn new() -> Self {
        let gen = MoveGenerator::new();
        let stats = Stats::new();
        MinimaxSearcher { gen, stats }
    }

    fn stats(&self) -> &Stats {
        &self.stats
    }

    fn best_move(&mut self, pos: &mut BoardState) -> EvaledMove {
        self.stats.reset();
        self.minimax(pos, 5)
    }

    fn best_move_depth(&mut self, pos: &mut BoardState, depth: usize) -> EvaledMove {
        self.stats.reset();
        self.minimax(pos, depth)
    }
}

impl MinimaxSearcher {
    fn minimax(&mut self, pos: &mut BoardState, depth: usize) -> EvaledMove {
        if depth == 0 {
            self.stats.count_node();
            return EvaledMove::null(eval(pos));
        }

        let moves = evaled_moves(self.gen.all_moves(pos));
        if moves.is_empty() {
            self.stats.count_node();
            return self.no_move_eval(pos, depth);
        }

        moves.into_iter().map(|mut mv: EvaledMove| {
            let mut new_pos = pos.clone_with_move(mv.mv);
            mv.eval = -self.minimax(&mut new_pos, depth - 1).eval;
            mv
        }).max().unwrap()
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
        let mut searcher: MinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 49)
    }

    #[test]
    fn finds_mate_in_one_as_black() {
        let mut pos = parse_fen(&"K7/8/2k5/8/8/8/8/1q6 b - - 0 1".to_string()).unwrap();
        let mut searcher: MinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 49)
    }

    #[test]
    fn best_move_random_1() {
        let mut pos =
            parse_fen(&"r2qkbnr/ppp2ppp/2np4/8/8/PPPpPbP1/7P/RNBQKBNR w KQkq - 0 8".to_string())
                .unwrap();
        let mut searcher: MinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 21)
    }

    #[test]
    fn best_move_random_2() {
        let mut pos =
            parse_fen(&"rnbqkbnr/7p/pppPpBp1/8/8/3P4/PPP2PPP/R2QKBNR b - - 0 1".to_string())
                .unwrap();
        let mut searcher: MinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 45)
    }

    #[test]
    fn best_move_random_3() {
        let mut pos =
            parse_fen(&"r2qkbnr/ppp2ppp/2np4/8/8/PPPpPbP1/7P/RNBQKBNR b KQkq - 0 8".to_string())
                .unwrap();
        let mut searcher: MinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        debug_print(&pos);
        assert_eq!(mv.to, 3)
    }

    #[test]
    fn best_move_random_4() {
        let mut pos = parse_fen(
            &"rnbqkbnr/1p1ppppp/2p5/8/p2PP2P/2N2N2/PPP2PP1/R1BQKB1R b KQkq - 0 5".to_string(),
        )
        .unwrap();
        let mut searcher: MinimaxSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos);
        println!("{}", mv.eval);
        println!("to: {}", mv.mv.to);
        println!("from: {}", mv.mv.from);
    }
}
