use crate::board_state::board::BoardState;
use crate::components::chess_move::Move;
use crate::components::piece::PieceType;
use crate::components::square::SquareIndex::{A1, D4};
use crate::magic::magic::MagicTable;
use crate::magic::random::MagicRandomizer;
use crate::magic::util::MagicPiece;
use crate::move_gen::lookup::Lookup;
use board_state::fen::*;
use move_gen::generator::gen_all_moves;
use rand::rngs::ThreadRng;
use std::time::Instant;

mod board_state;
mod components;
mod magic;
mod move_gen;

fn main() {
    println!("Hello, world!");
    let mut b = BoardState::default();
    let mut b = parse_fen(&"rnbqkbnr/1ppp1pp1/p6p/4p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR b KQkq - 0 1".to_string()).unwrap();
    gen_all_moves(&mut b, 1);
    println!();
}
