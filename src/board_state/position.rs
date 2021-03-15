use crate::board_state::player::*;
use crate::components::bitboard::*;
use crate::components::piece::{Color, Piece, PieceType};

pub struct Position {
    pub white: Player,
    pub black: Player,
}

impl Position {
    pub fn bb(&self, piece: PieceType, color: Color) -> Bitboard {
        match color {
            Color::White => self.white.bb(piece),
            Color::Black => self.black.bb(piece),
        }
    }

    pub fn bb_for_color(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white.bb_all(),
            Color::Black => self.black.bb_all(),
        }
    }

    pub fn add_piece(&mut self, piece: char, rank: u8, file: u8) {
        let piece = Piece::convert_char_to_piece(piece);
        match piece.color {
            Color::White => self.white.add_piece(piece, rank, file),
            Color::Black => self.black.add_piece(piece, rank, file),
        }
    }

    pub fn default() -> Position {
        let white = Player {
            pawns: RANK2,
            rooks: 0b10000001u64,
            knights: 0b01000010u64,
            bishops: 0b00100100u64,
            queen: 0b00010000u64,
            king: 0b00001000u64,
        };
        let black = Player {
            pawns: RANK7,
            rooks: 0b10000001u64 << (8 * 7),
            knights: 0b01000010u64 << (8 * 7),
            bishops: 0b00100100u64 << (8 * 7),
            queen: 0b00010000u64 << (8 * 7),
            king: 0b00001000u64 << (8 * 7),
        };
        Position { white, black }
    }

    pub fn empty() -> Position {
        let white = Player::default();
        let black = Player::default();
        Position { white, black }
    }
}
