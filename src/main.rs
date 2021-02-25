mod bitboard;
use bitboard::*;

fn main() {
    println!("Hello, world!");
    let b = Board::default();
    b.debug_print();
}
