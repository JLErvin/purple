use crate::components::bitboard::*;
use crate::components::piece::{Color, Piece, PieceType, COLOR_COUNT, PIECE_COUNT};

pub struct Position {
    pieces_bb: [Bitboard; PIECE_COUNT],
    colors_bb: [Bitboard; COLOR_COUNT],
}

impl Position {
    pub fn bb(&self, piece: PieceType, color: Color) -> Bitboard {
        self.pieces_bb[piece] & self.colors_bb[color]
    }

    pub fn bb_for_color(&self, color: Color) -> Bitboard {
        self.colors_bb[color]
    }

    pub fn add_piece(&mut self, c: char, rank: u8, file: u8) {
        let piece = Piece::convert_char_to_piece(c);
        let color = Piece::convert_char_to_color(c);
        self.pieces_bb[piece] = self.pieces_bb[piece].add_piece(rank, file);
        self.colors_bb[color] = self.colors_bb[color].add_piece(rank, file);
    }

    pub fn default() -> Position {
        let white = RANK1 | RANK2;
        let black = RANK7 | RANK8;
        let pawns = RANK2 | RANK7;
        let rooks = 0b10000001u64 | (0b10000001u64 << (8 * 7));
        let knights = 0b01000010u64 | (0b01000010u64 << (8 * 7));
        let bishops = 0b00100100u64 | (0b00100100u64 << (8 * 7));
        let queens = 0b00010000u64 | (0b00010000u64 << (8 * 7));
        let kings = 0b00001000u64 | (0b00001000u64 << (8 * 7));

        let mut pieces_bb: [Bitboard; PIECE_COUNT] = [0; PIECE_COUNT];
        pieces_bb[PieceType::Pawn] = pawns;
        pieces_bb[PieceType::Rook] = rooks;
        pieces_bb[PieceType::Knight] = knights;
        pieces_bb[PieceType::Bishop] = bishops;
        pieces_bb[PieceType::Queen] = queens;
        pieces_bb[PieceType::King] = kings;

        let mut colors_bb: [Bitboard; COLOR_COUNT] = [0; COLOR_COUNT];
        colors_bb[Color::White] = white;
        colors_bb[Color::Black] = black;

        Position {
            pieces_bb,
            colors_bb,
        }
    }

    pub fn empty() -> Position {
        let pieces_bb: [Bitboard; PIECE_COUNT] = [0; PIECE_COUNT];
        let colors_bb: [Bitboard; COLOR_COUNT] = [0; COLOR_COUNT];
        Position {
            pieces_bb,
            colors_bb,
        }
    }
}
