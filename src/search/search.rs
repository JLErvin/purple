use crate::board_state::board::BoardState;
use crate::common::chess_move::{Move, EAST, NORTH, SOUTH, WEST};
use crate::common::eval_move::EvaledMove;
use crate::common::stats::Stats;
use crate::move_gen::generator::MoveGenerator;
use crate::search::eval::{eval, no_move_eval, INF, NEG_INF};
use itertools::Itertools;

pub struct Searcher {
    gen: MoveGenerator,
    stats: Stats,
}

impl Searcher {
    pub fn new() -> Searcher {
        let gen = MoveGenerator::new();
        let stats = Stats::new();
        Searcher { gen, stats }
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    pub fn best_move(&mut self, pos: &mut BoardState) -> EvaledMove {
        self.stats.reset();
        self.alpha_beta(pos, NEG_INF, INF, 5)
    }

    fn alpha_beta(
        &mut self,
        pos: &mut BoardState,
        mut alpha: isize,
        beta: isize,
        depth: usize,
    ) -> EvaledMove {
        self.stats.count_node();
        if depth == 0 {
            self.stats.count_leaf_node();
            return EvaledMove::null(eval(pos));
        }

        let mut moves = evaled_moves(self.gen.all_moves(pos));

        if moves.is_empty() {
            self.stats.count_leaf_node();
            return no_move_eval(pos, depth);
        }

        let mut best_move = EvaledMove::null(alpha);
        for mv in moves.iter_mut() {
            let mut new_pos = pos.clone_with_move(mv.mv);
            mv.eval = -self.alpha_beta(&mut new_pos, -beta, -alpha, depth - 1).eval;
            if mv.eval > alpha {
                alpha = mv.eval;
                if alpha >= beta {
                    return *mv;
                }
                best_move = *mv;
            }
        }

        best_move
    }

    fn minimax(&self, pos: &mut BoardState, depth: usize) -> EvaledMove {
        if depth == 0 {
            return EvaledMove::null(eval(pos));
        }

        let moves = evaled_moves(self.gen.all_moves(pos));

        let best = moves
            .into_iter()
            .map(|mut mv: EvaledMove| {
                let mut new_pos = pos.clone_with_move(mv.mv);
                mv.eval = -self.minimax(&mut new_pos, depth - 1).eval;
                mv
            })
            .max();

        match best {
            None => no_move_eval(pos, depth),
            Some(mv) => mv,
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::board_state::fen::parse_fen;

    #[test]
    fn finds_mate_in_one_as_white() {
        let mut pos = parse_fen(&"k7/8/2K5/8/8/8/8/1Q6 w - - 0 1".to_string()).unwrap();
        let mut searcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 49)
    }

    #[test]
    fn finds_mate_in_one_as_black() {
        let mut pos = parse_fen(&"K7/8/2k5/8/8/8/8/1q6 b - - 0 1".to_string()).unwrap();
        let mut searcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 49)
    }
}
