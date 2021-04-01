use crate::components::bitboard::PieceItr;
use crate::components::chess_move::Move;
use crate::components::{bitboard::Bitboard, chess_move::MoveType};

pub fn extract_moves(bitboard: Bitboard, offset: i8, kind: MoveType, moves: &mut Vec<Move>) {
    for (square, bb) in bitboard.iter() {
        let m = Move {
            to: square as u8,
            from: (square as i8 - offset) as u8,
            kind,
        };
        moves.push(m);
    }
}
