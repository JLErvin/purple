use crate::board_state::board::BoardState;
use crate::components::bitboard::{Bitboard, ClearBit, GetBit, Shift, RANK3, RANK7};
use crate::components::chess_move::{Move, NORTH};
use crate::components::piece::PieceType;

use super::{kinggen::gen_pseudo_legal_king_moves, pawngen::gen_pseudo_legal_pawn_moves};
use crate::magic::random::MagicRandomizer;
use crate::move_gen::knightgen::gen_pseudo_legal_knight_moves;
use crate::move_gen::lookup::Lookup;
use crate::move_gen::slider::{
    gen_pseudo_legal_bishop_moves, gen_pseudo_legal_queen_moves, gen_pseudo_legal_rook_moves,
    gen_pseudo_legal_slider_moves,
};

const MAX_MOVES: usize = 256;

pub fn gen_all_pseduo_legal_moves(pos: &BoardState) {
    let random = MagicRandomizer::new();
    let lookup = Lookup::new(random);
    let mut list: Vec<Move> = Vec::with_capacity(MAX_MOVES);

    gen_pseudo_legal_pawn_moves(pos, &mut list);
    println!("Pawns {}", list.len());
    gen_pseudo_legal_king_moves(pos, &mut list, &lookup);
    println!("King {}", list.len());
    gen_pseudo_legal_knight_moves(pos, &mut list, &lookup);
    println!("Knight {}", list.len());
    //gen_pseudo_legal_slider_moves(pos, &mut list, &lookup, PieceType::Rook);
    gen_pseudo_legal_rook_moves(pos, &mut list, &lookup);
    println!("Rook {}", list.len());
    //gen_pseudo_legal_slider_moves(pos, &mut list, &lookup, PieceType::Bishop);
    gen_pseudo_legal_bishop_moves(pos, &mut list, &lookup);
    println!("Bishop {}", list.len());
    //gen_pseudo_legal_slider_moves(pos, &mut list, &lookup, PieceType::Queen);
    gen_pseudo_legal_queen_moves(pos, &mut list, &lookup);
    println!("Queen {}", list.len());

    println!("Number of moves: {}", list.len());
}
