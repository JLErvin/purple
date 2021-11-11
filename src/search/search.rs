use crate::board_state::board::BoardState;
use crate::common::chess_move::{Move, EAST, NORTH, SOUTH, WEST};
use crate::common::eval_move::EvaledMove;
use crate::common::piece::Color;
use crate::common::stats::Stats;
use crate::move_gen::generator::MoveGenerator;
use crate::search::eval::{eval, no_move_eval, INF, NEG_INF};
use itertools::Itertools;
use std::cmp::{max, min};


pub trait Searcher {
    fn new() -> Self;
    fn stats(&self) -> &Stats;
    fn best_move(&mut self, pos: &mut BoardState) -> EvaledMove;
    fn best_move_depth(&mut self, pos: &mut BoardState, depth: usize) -> EvaledMove;
}