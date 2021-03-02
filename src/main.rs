mod bitboard;
mod castle;
mod fen;
mod gamestate;
mod movegen;
mod p_move;
mod piece;
mod player;
mod position;
mod square;
use fen::*;
use gamestate::*;

fn main() {
    println!("Hello, world!");
    let b =
        parse_fen(&"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string());
    println!();
    b.unwrap().debug_print();
}
