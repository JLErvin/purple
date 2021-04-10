use crate::board_state::board::BoardState;
use crate::components::bitboard::{Bitboard, ClearBit, GetBit, Shift, RANK3, RANK7};
use crate::components::chess_move::{Move, NORTH};
use crate::components::piece::PieceType;

use super::pawns::gen_pseudo_legal_pawn_moves;
use crate::magic::random::{GenerationScheme, MagicRandomizer};
use crate::move_gen::legal::{cannot_move_because_pinned, is_legal_king, is_legal_king_in_check};
use crate::move_gen::lookup::Lookup;
use crate::move_gen::moves::gen_pseudo_legal_moves;
use itertools::Itertools;

const MAX_MOVES: usize = 256;

pub fn gen_all_pseudo_legal_moves(pos: &BoardState) {
    let random = MagicRandomizer::new(GenerationScheme::PreComputed);
    let lookup = Lookup::new(random);
    let mut list: Vec<Move> = Vec::with_capacity(MAX_MOVES);

    gen_pseudo_legal_pawn_moves(pos, &mut list);

    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::King);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Knight);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Rook);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Bishop);
    gen_pseudo_legal_moves(pos, &mut list, &lookup, PieceType::Queen);

    println!("Number of Pseudo-Legal Moves: {}", list.len());

    //let v = list
    //    .iter()
    //    .filter(|x| is_legal(pos, &x, &lookup))
    //    .collect_vec();

    let v = list
        .iter()
        .filter(|x| !cannot_move_because_pinned(pos, &x, &lookup))
        .filter(|x| is_legal_king(pos, &x, &lookup))
        .filter(|x| is_legal_king_in_check(pos, &x, &lookup))
        .collect_vec();

    println!("Number of legal moves: {} ", v.len());
}
