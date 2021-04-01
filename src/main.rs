use crate::board_state::board::BoardState;
use board_state::fen::*;
use move_gen::movegen::gen_all_pseduo_legal_moves;

mod board_state;
mod components;
mod move_gen;

fn main() {
    println!("Hello, world!");
    let b =
        parse_fen(&"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string());
    let b = b.unwrap();
    gen_all_pseduo_legal_moves(&b);
    println!();
}
