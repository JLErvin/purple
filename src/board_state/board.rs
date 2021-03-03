use crate::board_state::castle::Castle;
use crate::board_state::position::Position;
use crate::components::bitboard::*;
use crate::components::piece::*;
use crate::components::square::*;

pub struct BoardState {
    pub position: Position,
    pub active_player: Color,
    pub castling_rights: Castle,
    pub en_passant: Square,
    pub half_move: u8,
    pub full_move: u8,
}

impl BoardState {
    pub fn get_bb(&self, color: Color, piece: PieceType) -> Bitboard {
        self.position.get_pieces(piece, color)
    }

    pub fn all_bb_for_color(&self, color: Color) -> Bitboard {
        self.position.all_pieces(color)
    }

    pub fn all_bb(&self) -> Bitboard {
        self.position.all_pieces(Color::White) | self.position.all_pieces(Color::Black)
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
            en_passant: 0,
            half_move: 0,
            full_move: 0,
        }
    }

    pub fn default() -> BoardState {
        BoardState {
            position: Position::default(),
            active_player: Color::White,
            castling_rights: Castle::default(),
            en_passant: 0,
            half_move: 0,
            full_move: 0,
        }
    }

    pub fn pieces(&self) -> Vec<Bitboard> {
        let mut i = self.position.white.pieces();
        let j = self.position.black.pieces();
        i.extend(j);
        i
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
        assert_eq!(p.get_bb(Color::White, PieceType::Pawn), 65280);
        assert_eq!(p.get_bb(Color::White, PieceType::Rook), 129);
        assert_eq!(p.get_bb(Color::White, PieceType::Knight), 66);
        assert_eq!(p.get_bb(Color::White, PieceType::Bishop), 36);
        assert_eq!(p.get_bb(Color::White, PieceType::Queen), 16);
        assert_eq!(p.get_bb(Color::White, PieceType::King), 8);
        assert_eq!(p.get_bb(Color::Black, PieceType::Pawn), 71776119061217280);
        assert_eq!(p.get_bb(Color::Black, PieceType::Rook), 9295429630892703744);
        assert_eq!(
            p.get_bb(Color::Black, PieceType::Knight),
            4755801206503243776
        );
        assert_eq!(
            p.get_bb(Color::Black, PieceType::Bishop),
            2594073385365405696
        );
        assert_eq!(
            p.get_bb(Color::Black, PieceType::Queen),
            1152921504606846976
        );
        assert_eq!(p.get_bb(Color::Black, PieceType::King), 576460752303423488);
        //assert_eq!(p.white_to_move, true);
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
