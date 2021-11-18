use itertools::Itertools;
use std::cmp::{max, min};

use super::{
    eval::{eval, no_move_eval, INF, NEG_INF},
    search::Searcher,
};
use crate::{board_state::board::BoardState, common::{chess_move::Move, eval_move::EvaledMove, piece::Color, stats::Stats}, move_gen::generator::{MoveGenerator, debug_print}, table::{transposition::{Bound, Entry, TranspositionTable}, zobrist::ZobristTable}};

pub struct MinimaxTableSearcher {
    gen: MoveGenerator,
    stats: Stats,
    table: TranspositionTable,
    zobrist: ZobristTable,
}

impl Searcher for MinimaxTableSearcher {
    fn new() -> Self {
        let gen = MoveGenerator::new();
        let stats = Stats::new();
        let table = TranspositionTable::new_mb(5);
        let zobrist = ZobristTable::init();
        MinimaxTableSearcher {
            gen,
            stats,
            table,
            zobrist,
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
        self.minimax(pos, depth);
        let hash = self.zobrist.hash(pos);
        self.table.get(hash, 0).unwrap().best_move
    }
}

impl MinimaxTableSearcher {
    fn minimax(&mut self, pos: &mut BoardState, depth: usize) -> EvaledMove {
        let hash = self.zobrist.hash(pos);
        let cached_move = self.table.get(hash, depth);
        match cached_move {
            None => (),
            Some(m) => {
                if m.depth == depth as u8 && m.hash == hash {
                    return m.best_move
                }
            }
        };

        if depth == 0 {
            self.stats.count_node();
            let e = EvaledMove::null(eval(pos));
            let hash = self.zobrist.hash(pos);
            let entry = Entry {
                best_move: e,
                hash: hash,
                depth: depth as u8,
                bound: Bound::Exact
            };
            self.table.save(hash, entry, depth);

            return e;
        }

        let moves = evaled_moves(self.gen.all_moves(pos));
        if moves.is_empty() {
            self.stats.count_node();
            return no_move_eval(pos, depth);
        }

        let best_move = if pos.active_player() == Color::White {
            let mut best_move = EvaledMove::null(-INF);
            for mut mv in moves.into_iter() {
                let mut new_pos = pos.clone_with_move(mv.mv);
                mv.eval = self.minimax(&mut new_pos, depth - 1).eval;
                best_move = max(mv, best_move);
            }
            best_move
        } else {
            let mut best_move = EvaledMove::null(INF);
            for mut mv in moves.into_iter() {
                let mut new_pos = pos.clone_with_move(mv.mv);
                mv.eval = self.minimax(&mut new_pos, depth - 1).eval;
                best_move = min(best_move, mv);
            }
            best_move
        };

        let hash = self.zobrist.hash(pos);
        let entry = Entry {
            best_move: best_move,
            hash: hash,
            depth: depth as u8,
            bound: Bound::Exact
        };
        /*
        debug_print(pos);
        println!("{:?}", entry);
        println!();
        */
        self.table.save(hash, entry, depth);
        best_move
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
        let mut searcher: MinimaxTableSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 49)
    }

    #[test]
    fn finds_mate_in_one_as_black() {
        let mut pos = parse_fen(&"K7/8/2k5/8/8/8/8/1q6 b - - 0 1".to_string()).unwrap();
        let mut searcher: MinimaxTableSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 49)
    }

    #[test]
    fn best_move_random_1() {
        let mut pos =
            parse_fen(&"r2qkbnr/ppp2ppp/2np4/8/8/PPPpPbP1/7P/RNBQKBNR w KQkq - 0 8".to_string())
                .unwrap();
        let mut searcher: MinimaxTableSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 21)
    }

    #[test]
    fn best_move_random_2() {
        let mut pos =
            parse_fen(&"rnbqkbnr/7p/pppPpBp1/8/8/3P4/PPP2PPP/R2QKBNR b - - 0 1".to_string())
                .unwrap();
        let mut searcher: MinimaxTableSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos).mv;
        assert_eq!(mv.to, 45)
    }

    #[test]
    fn best_move_random_3() {
        let mut pos =
            parse_fen(&"r2qkbnr/ppp2ppp/2np4/8/8/PPPpPbP1/7P/RNBQKBNR b KQkq - 0 8".to_string())
                .unwrap();
        let mut searcher: MinimaxTableSearcher = Searcher::new();
        let mv = searcher.best_move(&mut pos);
        println!("{}", mv.eval);
        assert_eq!(mv.mv.to, 3)
    }
}
