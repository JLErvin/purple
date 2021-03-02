use crate::bitboard::*;

pub enum Color {
    Black,
    White,
}

pub struct Player {
    pub pawns: Bitboard,
    pub rooks: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub king: Bitboard,
    pub queen: Bitboard,
}

impl Player {
    pub fn get_all(&self) -> Bitboard {
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

    pub fn add_piece(&mut self, piece: char, rank: u8, file: u8) {
        let piece = piece.to_ascii_lowercase();
        match piece {
            'p' => self.pawns = self.pawns.add_piece(rank, file),
            'r' => self.rooks = self.rooks.add_piece(rank, file),
            'n' => self.knights = self.knights.add_piece(rank, file),
            'b' => self.bishops = self.bishops.add_piece(rank, file),
            'q' => self.queen = self.queen.add_piece(rank, file),
            'k' => self.king = self.king.add_piece(rank, file),
            _ => (),
        }
    }
}
