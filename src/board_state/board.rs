use crate::components::bitboard::*;
use crate::components::square::*;
use crate::components::square::*;
use crate::board_state::position::Position;
use crate::board_state::player::{Color, Player};
use crate::board_state::castle::Castle;

pub struct BoardState {
    pub position: Position,
    pub active_player: Color,
    pub castling_rights: Castle,
    pub en_passant: Square,
    pub half_move: u8,
    pub full_move: u8,
}

impl BoardState {
    pub fn add_piece(&mut self, piece: char, rank: u8, file: u8) {
        self.position.add_piece(piece, rank, file);
    }

    fn get_us(&self) -> &Player {
        match self.active_player {
            Color::White => &self.position.white,
            Color::Black => &self.position.black,
        }
    }

    fn get_them(&self) -> &Player {
        match self.active_player {
            Color::White => &self.position.black,
            Color::Black => &self.position.white,
        }
    }

    pub fn our_all(&self) -> Bitboard {
        self.get_us().get_all()
    }

    pub fn op_all(&self) -> Bitboard {
        self.get_them().get_all()
    }

    pub fn our_pawns(&self) -> Bitboard {
        self.get_us().pawns
    }

    pub fn our_rooks(&self) -> Bitboard {
        self.get_us().rooks
    }

    pub fn our_knights(&self) -> Bitboard {
        self.get_us().knights
    }

    pub fn our_bishops(&self) -> Bitboard {
        self.get_us().bishops
    }

    pub fn our_queen(&self) -> Bitboard {
        self.get_us().queen
    }

    pub fn our_king(&self) -> Bitboard {
        self.get_us().king
    }

    pub fn their_pawns(&self) -> Bitboard {
        self.get_them().pawns
    }

    pub fn their_rooks(&self) -> Bitboard {
        self.get_them().rooks
    }

    pub fn their_knights(&self) -> Bitboard {
        self.get_them().knights
    }

    pub fn their_bishops(&self) -> Bitboard {
        self.get_them().bishops
    }

    pub fn their_queen(&self) -> Bitboard {
        self.get_them().queen
    }

    pub fn their_king(&self) -> Bitboard {
        self.get_them().king
    }

    pub fn all(&self) -> Bitboard {
        self.get_us().get_all() | self.get_them().get_all()
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
        assert_eq!(p.our_pawns(), 65280);
        assert_eq!(p.our_rooks(), 129);
        assert_eq!(p.our_knights(), 66);
        assert_eq!(p.our_bishops(), 36);
        assert_eq!(p.our_queen(), 16);
        assert_eq!(p.our_king(), 8);
        assert_eq!(p.their_pawns(), 71776119061217280);
        assert_eq!(p.their_rooks(), 9295429630892703744);
        assert_eq!(p.their_knights(), 4755801206503243776);
        assert_eq!(p.their_bishops(), 2594073385365405696);
        assert_eq!(p.their_queen(), 1152921504606846976);
        assert_eq!(p.their_king(), 576460752303423488);
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
