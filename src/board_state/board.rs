use crate::board_state::castle::Castle;
use crate::board_state::position::Position;
use crate::components::bitboard::*;
use crate::components::chess_move::{Move, MoveType};
use crate::components::piece::PieceType::Rook;
use crate::components::piece::*;
use crate::components::square::SquareIndex::{
    A1, A8, C1, C8, D1, D8, E1, E8, F1, F8, G1, G8, H1, H8,
};
use crate::components::square::*;

#[derive(Copy, Clone)]
pub struct BoardState {
    pub(super) position: Position,
    pub(super) active_player: Color,
    pub(super) castling_rights: Castle,
    pub(super) en_passant: Option<Square>,
    pub(super) half_move: u8,
    pub(super) full_move: u8,
}

impl BoardState {
    #[inline]
    pub fn active_player(&self) -> Color {
        self.active_player
    }

    #[inline]
    pub fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    #[inline]
    pub fn castling_rights(&self) -> &Castle {
        &self.castling_rights
    }

    #[inline]
    pub fn half_move(&self) -> u8 {
        self.half_move
    }

    #[inline]
    pub fn full_move(&self) -> u8 {
        self.full_move
    }

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

    pub fn make_move(&mut self, mv: Move) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::square::SquareIndex::{A2, A3, A4, B1, B4, B8, C3, C6, D2, D3, D4};

    #[test]
    fn correct_initial_values() {
        let p = BoardState::default();
        assert_eq!(p.bb(Color::White, PieceType::Pawn), 65280);
        assert_eq!(p.bb(Color::White, PieceType::Rook), 129);
        assert_eq!(p.bb(Color::White, PieceType::Knight), 66);
        assert_eq!(p.bb(Color::White, PieceType::Bishop), 36);
        assert_eq!(p.bb(Color::White, PieceType::Queen), 8);
        assert_eq!(p.bb(Color::White, PieceType::King), 16);
        assert_eq!(p.bb(Color::Black, PieceType::Pawn), 71776119061217280);
        assert_eq!(p.bb(Color::Black, PieceType::Rook), 9295429630892703744);
        assert_eq!(p.bb(Color::Black, PieceType::Knight), 4755801206503243776);
        assert_eq!(p.bb(Color::Black, PieceType::Bishop), 2594073385365405696);
        assert_eq!(p.bb(Color::Black, PieceType::Queen), 576460752303423488);
        assert_eq!(p.bb(Color::Black, PieceType::King), 1152921504606846976);
        assert_eq!(p.active_player(), Color::White);
        assert_eq!(p.half_move(), 0);
        assert_eq!(p.full_move(), 1);
        assert_eq!(p.castling_rights().black_king, true);
        assert_eq!(p.castling_rights().black_queen, true);
        assert_eq!(p.castling_rights().white_king, true);
        assert_eq!(p.castling_rights().white_queen, true);
    }

    #[test]
    fn correct_empty_values() {
        let p = BoardState::empty();
        assert_eq!(p.bb(Color::White, PieceType::Pawn), 0);
        assert_eq!(p.bb(Color::White, PieceType::Rook), 0);
        assert_eq!(p.bb(Color::White, PieceType::Knight), 0);
        assert_eq!(p.bb(Color::White, PieceType::Bishop), 0);
        assert_eq!(p.bb(Color::White, PieceType::Queen), 0);
        assert_eq!(p.bb(Color::White, PieceType::King), 0);
        assert_eq!(p.bb(Color::Black, PieceType::Pawn), 0);
        assert_eq!(p.bb(Color::Black, PieceType::Rook), 0);
        assert_eq!(p.bb(Color::Black, PieceType::Knight), 0);
        assert_eq!(p.bb(Color::Black, PieceType::Bishop), 0);
        assert_eq!(p.bb(Color::Black, PieceType::Queen), 0);
        assert_eq!(p.bb(Color::Black, PieceType::King), 0);
        assert_eq!(p.active_player(), Color::White);
        assert_eq!(p.half_move(), 0);
        assert_eq!(p.full_move(), 0);
        assert_eq!(p.castling_rights().black_king, true);
        assert_eq!(p.castling_rights().black_queen, true);
        assert_eq!(p.castling_rights().white_king, true);
        assert_eq!(p.castling_rights().white_queen, true);
    }

    #[test]
    fn correctly_adds_piece() {
        let mut p = BoardState::empty();
        assert_eq!(p.bb(Color::White, PieceType::Pawn), 0);
        p.add_piece('P', 0, 0);
        assert_eq!(p.bb(Color::White, PieceType::Pawn), 1);
    }

    #[test]
    fn gets_bit_lsb() {
        let b: Bitboard = 0b0000_0001u64;
        let b1 = b.get_bit_lsb(0);
        let b2 = b.get_bit_lsb(63);
        assert_eq!(b1 as u8, 1);
        assert_eq!(b2 as u8, 0);
    }

    #[test]
    fn makes_pawn_push() {
        let mut b = BoardState::default();
        let m = Move {
            to: D4 as u8,
            from: D2 as u8,
            kind: MoveType::Quiet,
        };
        b.make_move(m);
        assert_eq!(b.bb_all(), 18446462598867122175);
        assert_eq!(b.en_passant, Some(D3 as u8));
        let m = Move {
            to: C6 as u8,
            from: B8 as u8,
            kind: MoveType::Quiet,
        };
        b.make_move(m);
        assert_eq!(b.en_passant, None)
    }

    #[test]
    fn makes_knight_move() {
        let mut b = BoardState::default();
        let m = Move {
            to: C3 as u8,
            from: B1 as u8,
            kind: MoveType::Quiet,
        };
        b.make_move(m);
        assert_eq!(b.bb_all(), 18446462598733168637);
    }
}
