use crate::bitboard::{AddPiece, Bitboard, ClearBit, GetBit, INIT_W_BISHOPS, INIT_W_KING, INIT_W_KNIGHTS, INIT_W_QUEEN, INIT_W_ROOKS, New, PieceItr, RANK1, RANK2, RANK7, RANK8, Shift};
use crate::chess_move::{Move, MoveType};
use crate::piece::PieceType::Rook;
use crate::piece::{Color, PieceType, COLOR_COUNT, PIECE_COUNT, Piece};
use crate::square::Square;
use crate::square::SquareIndex::{A1, A8, C1, C8, D1, D8, E1, E8, F1, F8, G1, G8, H1, H8};

#[derive(Copy, Clone)]
pub struct BoardState {
    pub position: Position,
    pub active_player: Color,
    pub castling_rights: Castle,
    pub en_passant: Option<Square>,
    pub half_move: u8,
    pub full_move: u8,
}

impl BoardState {
    #[inline]
    pub fn bb(&self, color: Color, piece: PieceType) -> Bitboard {
        self.position.bb(piece, color)
    }

    #[inline]
    pub fn bb_for_color(&self, color: Color) -> Bitboard {
        self.position.bb_for_color(color)
    }

    #[inline]
    pub fn bb_pieces(&self, piece: PieceType) -> Bitboard {
        self.position.bb_for_piece(piece)
    }

    #[inline]
    pub fn bb_all(&self) -> Bitboard {
        self.position.bb_for_color(Color::White) | self.position.bb_for_color(Color::Black)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn add_piece(&mut self, piece: char, rank: u8, file: u8) {
        self.position.add_piece(piece, rank, file);
    }

    #[inline]
    pub fn remove_piece(&mut self, piece: PieceType, color: Color, square: Square) {
        self.position.remove(piece, color, square);
    }

    #[inline]
    pub fn add(&mut self, piece: PieceType, color: Color, square: Square) {
        self.position.add(piece, color, square);
    }

    #[inline]
    pub fn switch(&mut self) {
        self.active_player = !self.active_player;
    }

    #[inline]
    pub fn type_on(&self, square: Square) -> Option<PieceType> {
        self.position.type_on(square)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn color_on(&self, square: Square) -> Option<Color> {
        self.position.color_on(square)
    }

    pub fn clone_with_move(&self, mv: Move) -> BoardState {
        let mut new_pos = *self;
        new_pos.make_move(mv);
        new_pos
    }

    pub fn make_move(&mut self, mv: Move) {
        if mv.kind == MoveType::Null {
            return;
        }

        let kind = self.position.type_on(mv.from).unwrap();
        let us = self.active_player;

        if kind == PieceType::King {
            self.castling_rights.remove_rights(us);
        }

        if kind == PieceType::Pawn {
            if mv.is_double_pawn_push() {
                self.make_double_push(&mv);
            } else {
                self.en_passant = None;
            }
        } else {
            self.en_passant = None;
        }

        if kind == PieceType::Rook {
            self.make_rook_move(mv);
        }

        let ep_offset: i8 = match us {
            Color::White => 8,
            Color::Black => -8,
        };

        if mv.kind == MoveType::Quiet {
            self.remove_piece(kind, us, mv.from);
            self.add(kind, us, mv.to);
        } else if mv.kind == MoveType::Capture {
            self.capture(mv, us);
        } else if mv.kind == MoveType::EnPassantCapture {
            self.remove_piece(kind, us, mv.from);
            self.remove_piece(kind, !us, (mv.to as i8 - ep_offset) as u8);
            self.add(kind, us, mv.to);
        } else if mv.is_promotion() {
            self.remove_piece(kind, us, mv.from);
            let add = mv.promoted_piece().unwrap();
            self.add(add, us, mv.to);
        } else if mv.is_promotion_capture() {
            let capture_kind = self.position.type_on(mv.to).unwrap();

            if capture_kind == Rook {
                self.capture_rook(mv, self.active_player);
            }

            self.remove_piece(kind, us, mv.from);
            self.remove_piece(capture_kind, !us, mv.to);
            let add = mv.promoted_piece().unwrap();
            self.add(add, us, mv.to);
        } else if mv.is_castle() {
            self.position.castle(mv.kind, self.active_player);
            self.castling_rights.remove_rights(self.active_player);
        }
        self.switch();
    }

    fn make_double_push(&mut self, mv: &Move) {
        match self.active_player {
            Color::White => self.en_passant = Some(mv.to - 8),
            Color::Black => self.en_passant = Some(mv.to + 8),
        }
    }

    fn capture(&mut self, mv: Move, active: Color) {
        let captured = self.type_on(mv.to).unwrap();
        if captured == PieceType::Rook {
            self.capture_rook(mv, active);
        }
        self.position.capture(mv, self.active_player);
    }

    fn capture_rook(&mut self, mv: Move, active: Color) {
        match active {
            Color::White => {
                if mv.to == H8 as u8 {
                    self.castling_rights.black_king = false;
                } else if mv.to == A8 as u8 {
                    self.castling_rights.black_queen = false;
                }
            }
            Color::Black => {
                if mv.to == H1 as u8 {
                    self.castling_rights.white_king = false;
                } else if mv.to == A1 as u8 {
                    self.castling_rights.white_queen = false;
                }
            }
        }
    }

    fn make_rook_move(&mut self, mv: Move) {
        if self.active_player == Color::White {
            if mv.from == H1 as u8 {
                self.castling_rights.white_king = false;
            }
            if mv.from == A1 as u8 {
                self.castling_rights.white_queen = false;
            }
        } else {
            if mv.from == H8 as u8 {
                self.castling_rights.black_king = false;
            }
            if mv.from == A8 as u8 {
                self.castling_rights.black_queen = false;
            }
        }
    }

    #[allow(dead_code)]
    pub fn empty() -> BoardState {
        let position = Position::empty();
        BoardState {
            position,
            active_player: Color::White,
            castling_rights: Castle::default(),
            en_passant: None,
            half_move: 0,
            full_move: 0,
        }
    }

    pub fn default() -> BoardState {
        BoardState {
            position: Position::default(),
            active_player: Color::White,
            castling_rights: Castle::default(),
            en_passant: None,
            half_move: 0,
            full_move: 1,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Castle {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

impl Castle {
    pub fn remove_rights(&mut self, color: Color) {
        match color {
            Color::White => {
                self.white_king = false;
                self.white_queen = false;
            }
            Color::Black => {
                self.black_king = false;
                self.black_queen = false;
            }
        }
    }

    pub fn default() -> Castle {
        Castle {
            white_king: true,
            white_queen: true,
            black_king: true,
            black_queen: true,
        }
    }
}

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
