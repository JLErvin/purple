use crate::components::bitboard::*;
use crate::components::piece::{Piece, PieceType};

pub struct Player {
    pub pawns: Bitboard,
    pub rooks: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub king: Bitboard,
    pub queen: Bitboard,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            pawns: 0,
            rooks: 0,
            knights: 0,
            bishops: 0,
            king: 0,
            queen: 0,
        }
    }
}

impl Player {
    pub fn bb(&self, piece: PieceType) -> Bitboard {
        match piece {
            PieceType::Pawn => self.pawns,
            PieceType::Rook => self.rooks,
            PieceType::Knight => self.knights,
            PieceType::Bishop => self.bishops,
            PieceType::King => self.king,
            PieceType::Queen => self.queen,
        }
    }

    pub fn bb_all(&self) -> Bitboard {
        self.pawns | self.rooks | self.knights | self.bishops | self.king | self.queen
    }

    pub fn pieces(&self) -> Vec<Bitboard> {
        vec![
            self.pawns,
            self.rooks,
            self.knights,
            self.bishops,
            self.king,
            self.queen,
        ]
    }

    pub fn add_piece(&mut self, piece: Piece, rank: u8, file: u8) {
        match piece.piece_type {
            PieceType::Pawn => self.pawns = self.pawns.add_piece(rank, file),
            PieceType::Rook => self.rooks = self.rooks.add_piece(rank, file),
            PieceType::Knight => self.knights = self.knights.add_piece(rank, file),
            PieceType::Bishop => self.bishops = self.bishops.add_piece(rank, file),
            PieceType::Queen => self.queen = self.queen.add_piece(rank, file),
            PieceType::King => self.king = self.king.add_piece(rank, file),
        }
    }
}
