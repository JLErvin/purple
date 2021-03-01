mod position;
mod bitboard;
mod movegen;
mod p_move;
mod piece;
mod fen;
mod player;
use position::*;
use fen::*;

fn main() {
    println!("Hello, world!");
    //let b = parse_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    //let b = parse_fen(&"rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string());
    let b = parse_fen(&"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string());
    println!();
    b.unwrap().debug_print();
}
