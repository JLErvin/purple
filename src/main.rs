use crate::board_state::board::BoardState;
use crate::components::chess_move::Move;
use crate::components::piece::PieceType;
use crate::components::square::SquareIndex::{A1, D4};
use crate::magic::magicgen::MagicTable;
use crate::magic::random::MagicRandomizer;
use crate::magic::util::MagicPiece;
use crate::move_gen::lookup::Lookup;
use board_state::fen::*;
use move_gen::movegen::gen_all_pseudo_legal_moves;
use rand::rngs::ThreadRng;
use std::time::Instant;

mod board_state;
mod components;
mod magic;
mod move_gen;

fn main() {
    println!("Hello, world!");
    let b = parse_fen(&"RB6/p6p/1P1p1Pb1/4Pn1k/B2p1K2/2N5/p7/8 w - - 0 1".to_string());
    let tic = Instant::now();

    let pos = b.unwrap();
    gen_all_pseudo_legal_moves(&pos);

    /*    let mut r = MagicRandomizer::new();
        let t = Lookup::new(r);
        let square = D4 as u8;
        let blockers = 2251800920983552u64;
        let mut list: Vec<Move> = Vec::with_capacity(256);
        let moves = t.rook_moves(square, blockers);
        gen_pseudo_legal_rook_moves(&b.unwrap(), &mut list, &t);

        println!("Desired length of moves: {}", 15);
        println!("Actual length of moves : {}", list.len());

        println!();

        println!("Desired: {}", 2260632246683648u64);
        println!("Actual : {}", moves);
    */
    let toc = tic.elapsed().as_secs_f64();
    //println!("{}", n);
    println!("Took {} seconds", toc);
    println!();
}
