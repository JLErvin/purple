use crate::components::bitboard::Bitboard;
use crate::components::square::Square;
use crate::magic::magicgen::MagicTable;
use crate::magic::util::{rook_ray, MagicPiece};

pub struct Lookup {
    rook_table: MagicTable,
    bishop_table: MagicTable,
}

impl Lookup {
    pub fn new() -> Lookup {
        let rook_table = MagicTable::init(MagicPiece::Rook);
        let bishop_table = MagicTable::init(MagicPiece::Bishop);

        Lookup {
            rook_table,
            bishop_table,
        }
    }

    pub fn rook_moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        self.rook_table.moves(square, blockers)
    }

    pub fn bishop_moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        self.bishop_table.moves(square, blockers)
    }
}
