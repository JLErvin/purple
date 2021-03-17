use crate::board_state::castle::Castle;
use crate::board_state::position::Position;
use crate::components::bitboard::*;
use crate::components::piece::*;
use crate::components::square::*;

pub struct BoardState {
    pub(super) position: Position,
    pub(super) active_player: Color,
    pub(super) castling_rights: Castle,
    pub(super) en_passant: Option<Square>,
    pub(super) half_move: u8,
    pub(super) full_move: u8,
}

impl BoardState {
    pub fn active_player(&self) -> Color {
        self.active_player
    }

    pub fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    pub fn castling_rights(&self) -> &Castle {
        &self.castling_rights
    }

    pub fn half_move(&self) -> u8 {
        self.half_move
    }

    pub fn full_move(&self) -> u8 {
        self.full_move
    }

    pub fn bb(&self, color: Color, piece: PieceType) -> Bitboard {
        self.position.bb(piece, color)
    }

    pub fn bb_for_color(&self, color: Color) -> Bitboard {
        self.position.bb_for_color(color)
    }

    pub fn bb_all(&self) -> Bitboard {
        self.position.bb_for_color(Color::White) | self.position.bb_for_color(Color::Black)
    }

    pub fn add_piece(&mut self, piece: char, rank: u8, file: u8) {
        self.position.add_piece(piece, rank, file);
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

    fn pieces(&self) -> Vec<Bitboard> {
        let a = Vec::new();
        a
    }

    pub fn debug_print(&self) {
        let mut p: u64 = 0;
        for b in self.pieces() {
            p |= b
        }
        for i in 1..65 {
            print!("{}", p.get_bit_msb(i - 1) as u64);
            if i % 8 == 0 {
                println!();
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_initial_values() {
        let p = BoardState::default();
        assert_eq!(p.bb(Color::White, PieceType::Pawn), 65280);
        assert_eq!(p.bb(Color::White, PieceType::Rook), 129);
        assert_eq!(p.bb(Color::White, PieceType::Knight), 66);
        assert_eq!(p.bb(Color::White, PieceType::Bishop), 36);
        assert_eq!(p.bb(Color::White, PieceType::Queen), 16);
        assert_eq!(p.bb(Color::White, PieceType::King), 8);
        assert_eq!(p.bb(Color::Black, PieceType::Pawn), 71776119061217280);
        assert_eq!(p.bb(Color::Black, PieceType::Rook), 9295429630892703744);
        assert_eq!(p.bb(Color::Black, PieceType::Knight), 4755801206503243776);
        assert_eq!(p.bb(Color::Black, PieceType::Bishop), 2594073385365405696);
        assert_eq!(p.bb(Color::Black, PieceType::Queen), 1152921504606846976);
        assert_eq!(p.bb(Color::Black, PieceType::King), 576460752303423488);
        assert_eq!(p.active_player(), Color::White);
        assert_eq!(p.half_move(), 0);
        assert_eq!(p.full_move(), 1);
        assert_eq!(p.castling_rights().black_king, true);
        assert_eq!(p.castling_rights().black_queen, true);
        assert_eq!(p.castling_rights().white_king, true);
        assert_eq!(p.castling_rights().white_queen, true);
    }

    #[test]
    fn gets_bit_lsb() {
        let b: Bitboard = 0b0000_0001u64;
        let b1 = b.get_bit_lsb(0);
        let b2 = b.get_bit_lsb(63);
        assert_eq!(b1 as u8, 1);
        assert_eq!(b2 as u8, 0);
    }
}
