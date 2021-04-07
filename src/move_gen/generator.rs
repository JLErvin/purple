use crate::board_state::board::BoardState;
use crate::components::bitboard::{Bitboard, ClearBit, GetBit, Shift, RANK3, RANK7};
use crate::components::chess_move::{Move, NORTH};
use crate::components::piece::PieceType;

use super::pawns::gen_pseudo_legal_pawn_moves;
use crate::magic::random::MagicRandomizer;
use crate::move_gen::lookup::Lookup;
use crate::move_gen::moves::gen_pseudo_legal_moves;

const MAX_MOVES: usize = 256;

pub fn gen_all_pseudo_legal_moves(pos: &BoardState) {
    let random = MagicRandomizer::new();
    let lookup = Lookup::new(random);
    let mut list: Vec<Move> = Vec::with_capacity(MAX_MOVES);

    gen_pseudo_legal_pawn_moves(pos, &mut list);

    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::King);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Knight);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Rook);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Bishop);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Queen);

    println!("Number of moves: {}", list.len());
}
