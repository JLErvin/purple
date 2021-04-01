use crate::board_state::board::BoardState;
use crate::components::bitboard::{Bitboard, ClearBit, GetBit, Shift, RANK3, RANK7};
use crate::components::chess_move::{Move, NORTH};
use crate::components::piece::PieceType;

use super::{kinggen::gen_pseudo_legal_king_moves, pawngen::gen_pseudo_legal_pawn_moves};

const MAX_MOVES: usize = 256;

pub fn gen_all_pseduo_legal_moves(pos: &BoardState) {
    let mut list: Vec<Move> = Vec::with_capacity(MAX_MOVES);
    gen_pseudo_legal_pawn_moves(pos, &mut list);
    gen_pseudo_legal_king_moves(pos, &mut list)
}