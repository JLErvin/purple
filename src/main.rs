use crate::board_state::board::BoardState;
use crate::components::piece::PieceType;
use crate::magic::magicgen::{MagicGenerator, MagicTable};
use crate::magic::util::MagicPiece;
use board_state::fen::*;
use move_gen::movegen::gen_all_pseduo_legal_moves;
use rand::rngs::ThreadRng;
use std::time::Instant;

mod board_state;
mod components;
mod magic;
mod move_gen;

fn main() {
    println!("Hello, world!");
    let b =
        parse_fen(&"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string());
    let tic = Instant::now();
    let m = MagicTable::init(MagicPiece::Rook);
    let m = MagicTable::init(MagicPiece::Bishop);
    let toc = tic.elapsed().as_secs_f64();
    //println!("{}", n);
    println!("Took {} seconds", toc);
    println!();
}
