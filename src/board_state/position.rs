use crate::board_state::player::*;
use crate::components::bitboard::*;
use crate::components::piece::{Color, Piece};

pub struct Position {
    pub white: Player,
    pub black: Player,
}

impl Position {
    pub fn get_pieces(&self, piece: &Piece, color: &Color) -> Bitboard {
        match color {
            Color::White => self.white.get_piece(piece),
            Color::Black => self.black.get_piece(piece),
        }
    }

    pub fn all_pieces(&self, color: &Color) -> Bitboard {
        match color {
            Color::White => self.white.get_all(),
            Color::Black => self.black.get_all(),
        }
    }

    pub fn add_piece(&mut self, piece: char, rank: u8, file: u8) {
        let player: &mut Player;
        if piece.is_lowercase() {
            player = &mut self.black;
        } else {
            player = &mut self.white;
        }
        player.add_piece(piece, rank, file);
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
        let white = Player {
            pawns: 0,
            rooks: 0,
            knights: 0,
            bishops: 0,
            queen: 0,
            king: 0,
        };
        let black = Player {
            pawns: 0,
            rooks: 0,
            knights: 0,
            bishops: 0,
            queen: 0,
            king: 0,
        };
        Position { white, black }
    }
}
