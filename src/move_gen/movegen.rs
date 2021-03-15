use crate::board_state::board::BoardState;
use crate::components::bitboard::{Bitboard, GetBit, Shift, RANK7};
use crate::components::chess_move::UP;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doesnt_crash() {}
}
