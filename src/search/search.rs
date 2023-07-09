use crate::board::BoardState;
use crate::chess_move::EvaledMove;
use crate::search::stats::Stats;

pub trait Searcher {
    fn new() -> Self;
    fn stats(&self) -> &Stats;
    fn best_move(&mut self, pos: &mut BoardState) -> EvaledMove;
    fn best_move_depth(&mut self, pos: &mut BoardState, depth: usize) -> EvaledMove;
    fn move_time(&mut self, seconds: u128);
}
