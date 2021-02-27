mod position;
mod bitboard;
mod movegen;
mod p_move;
mod piece;
use position::*;

fn main() {
    println!("Hello, world!");
    let b = Position::default();
    b.debug_print();
}
