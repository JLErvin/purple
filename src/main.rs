use crate::board_state::board::BoardState;
use crate::components::chess_move::Move;
use crate::components::piece::PieceType;
use crate::components::square::SquareIndex::{A1, D4};
use crate::magic::magic::MagicTable;
use crate::magic::random::MagicRandomizer;
use crate::magic::util::MagicPiece;
use crate::move_gen::lookup::Lookup;
use board_state::fen::*;
use move_gen::generator::gen_all_pseudo_legal_moves;
use rand::rngs::ThreadRng;
use std::time::Instant;

mod board_state;
mod components;
mod magic;
mod move_gen;

fn main() {
    println!("Hello, world!");
    let mut b = parse_fen(
        &"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1

"
        .to_string(),
    )
    .unwrap();
    gen_all_pseudo_legal_moves(&mut b, 6);
    println!();
}
