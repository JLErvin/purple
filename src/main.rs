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
use std::env;
use std::time::Instant;

mod board_state;
mod components;
mod magic;
mod move_gen;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let depth: usize = args[1].parse::<usize>().unwrap();
    let fen_str = &args[2];

    let mut a = parse_fen(fen_str).unwrap();

    let mut b = BoardState::default();
    gen_all_moves(&mut b, depth);
    println!();
}
