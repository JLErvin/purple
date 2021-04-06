use crate::components::bitboard::*;
use crate::components::piece::{Color, Piece, PieceType, COLOR_COUNT, PIECE_COUNT};

pub struct Position {
    pieces_bb: [Bitboard; PIECE_COUNT],
    colors_bb: [Bitboard; COLOR_COUNT],
}

impl Position {
    #[inline]
    pub fn bb(&self, piece: PieceType, color: Color) -> Bitboard {
        self.pieces_bb[piece] & self.colors_bb[color]
    }

    #[inline]
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
        let mut pieces_bb: [Bitboard; PIECE_COUNT] = [0; PIECE_COUNT];
        pieces_bb[PieceType::Pawn] = RANK2 | RANK7;
        pieces_bb[PieceType::Rook] = INIT_W_ROOKS | INIT_W_ROOKS.shift(8 * 7);
        pieces_bb[PieceType::Knight] = INIT_W_KNIGHTS | INIT_W_KNIGHTS.shift(8 * 7);
        pieces_bb[PieceType::Bishop] = INIT_W_BISHOPS | INIT_W_BISHOPS.shift(8 * 7);
        pieces_bb[PieceType::Queen] = INIT_W_QUEEN | INIT_W_QUEEN.shift(8 * 7);
        pieces_bb[PieceType::King] = INIT_W_KING | INIT_W_KING.shift(8 * 7);

        let mut colors_bb: [Bitboard; COLOR_COUNT] = [0; COLOR_COUNT];
        colors_bb[Color::White] = RANK1 | RANK2;
        colors_bb[Color::Black] = RANK7 | RANK8;

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
