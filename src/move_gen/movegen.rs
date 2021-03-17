use crate::board_state::board::BoardState;
use crate::components::bitboard::{Bitboard, GetBit, Shift, RANK7};
use crate::components::chess_move::{Move, UP};
use crate::components::piece::PieceType;

const MAX_MOVES: usize = 256;

pub fn gen_pawn_moves(pos: BoardState) {
    gen_single_pawn_moves(&pos);
}

fn gen_single_pawn_moves(pos: &BoardState) {
    let us = pos.active_player();
    let pawns = pos.bb(us, PieceType::Pawn) & !RANK7;
    let empty_squares = !pos.bb_all();
    let forward = pawns.shift(UP) & empty_squares;
}

fn extract_pawn_moves(mut bitboard: Bitboard) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    while bitboard != 0 {
        let index = bitboard.trailing_zeros();
        bitboard &= !(1 << index);
        let m = Move {
            to: index as u8,
            from: (index - 8) as u8,
        };
        moves.push(m);
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::bitboard::RANK2;

    #[test]
    fn extract_basic_pawn_moves() {
        let b = extract_pawn_moves(RANK2);
        assert_eq!(b.len(), 8);
        assert_eq!(b.get(0).unwrap().to, 8);
        assert_eq!(b.get(1).unwrap().to, 9);
    }
}
