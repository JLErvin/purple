use crate::components::bitboard::Bitboard;
use crate::magic::util::rook_ray;

struct Lookup {
    king_moves: [Bitboard; 64],
    knight_moves: [Bitboard; 64],
}

impl Lookup {}
