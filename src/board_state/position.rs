use crate::common::bitboard::*;
use crate::common::chess_move::{Move, MoveType};
use crate::common::piece::{Color, Piece, PieceType, COLOR_COUNT, PIECE_COUNT};
use crate::common::square::Square;
use crate::common::square::SquareIndex::{A1, A8, C1, C8, D1, D8, E1, E8, F1, F8, G1, G8, H1, H8};

#[derive(Copy, Clone)]
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

    #[inline]
    pub fn bb_for_piece(&self, piece: PieceType) -> Bitboard {
        self.pieces_bb[piece]
    }

    pub fn add_piece(&mut self, c: char, rank: u8, file: u8) {
        let piece = Piece::convert_char_to_piece(c);
        let color = Piece::convert_char_to_color(c);
        self.pieces_bb[piece] = self.pieces_bb[piece].add_piece(rank, file);
        self.colors_bb[color] = self.colors_bb[color].add_piece(rank, file);
    }

    pub fn add(&mut self, piece: PieceType, color: Color, square: Square) {
        self.pieces_bb[piece] = self.pieces_bb[piece].add_at_square(square);
        self.colors_bb[color] = self.colors_bb[color].add_at_square(square);
    }

    pub fn remove(&mut self, piece: PieceType, color: Color, square: Square) {
        self.pieces_bb[piece] = self.pieces_bb[piece].clear_bit(square);
        self.colors_bb[color] = self.colors_bb[color].clear_bit(square);
    }

    pub fn castle(&mut self, kind: MoveType, color: Color) {
        match kind {
            MoveType::CastleKing => self.castle_king(color),
            MoveType::CastleQueen => self.castle_queen(color),
            _ => {}
        }
    }

    pub fn capture(&mut self, mv: Move, active: Color) {
        let captured = self.type_on(mv.to).unwrap();
        let kind = self.type_on(mv.from).unwrap();
        self.remove(kind, active, mv.from);
        self.remove(captured, !active, mv.to);
        self.add(kind, active, mv.to);
    }

    fn castle_king(&mut self, color: Color) {
        match color {
            Color::White => {
                self.remove(PieceType::King, color, E1 as u8);
                self.remove(PieceType::Rook, color, H1 as u8);
                self.add(PieceType::King, color, G1 as u8);
                self.add(PieceType::Rook, color, F1 as u8);
            }
            Color::Black => {
                self.remove(PieceType::King, color, E8 as u8);
                self.remove(PieceType::Rook, color, H8 as u8);
                self.add(PieceType::King, color, G8 as u8);
                self.add(PieceType::Rook, color, F8 as u8);
            }
        }
    }

    fn castle_queen(&mut self, color: Color) {
        match color {
            Color::White => {
                self.remove(PieceType::King, color, E1 as u8);
                self.remove(PieceType::Rook, color, A1 as u8);
                self.add(PieceType::King, color, C1 as u8);
                self.add(PieceType::Rook, color, D1 as u8);
            }
            Color::Black => {
                self.remove(PieceType::King, color, E8 as u8);
                self.remove(PieceType::Rook, color, A8 as u8);
                self.add(PieceType::King, color, C8 as u8);
                self.add(PieceType::Rook, color, D8 as u8);
            }
        }
    }

    pub fn type_on(&self, square: Square) -> Option<PieceType> {
        let piece_bb = Bitboard::for_square(square);
        for (i, bb) in self.pieces_bb.iter().enumerate() {
            if piece_bb & *bb != 0 {
                match i {
                    0 => return Some(PieceType::Pawn),
                    1 => return Some(PieceType::Rook),
                    2 => return Some(PieceType::Knight),
                    3 => return Some(PieceType::Bishop),
                    4 => return Some(PieceType::Queen),
                    5 => return Some(PieceType::King),
                    _ => return None,
                };
            }
        }
        None
    }

    pub fn color_on(&self, square: Square) -> Option<Color> {
        if self.colors_bb[Color::White].get_bit_lsb(square as i8) {
            return Some(Color::White);
        }
        if self.colors_bb[Color::Black].get_bit_lsb(square as i8) {
            return Some(Color::Black);
        }
        None
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
