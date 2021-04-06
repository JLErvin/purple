use crate::board_state::board::BoardState;
use crate::board_state::position::Position;
use crate::components::bitboard::{Bitboard, PieceItr};
use crate::components::chess_move::MoveType::{Capture, Quiet};
use crate::components::chess_move::{Move, MoveType};
use crate::components::piece::PieceType;
use crate::move_gen::lookup::Lookup;
use crate::move_gen::util::extract_moves;
use std::io::empty;

pub fn gen_pseudo_legal_slider_moves(
    pos: &BoardState,
    list: &mut Vec<Move>,
    lookup: &Lookup,
    piece: PieceType,
) {
    let us = pos.active_player();
    let pieces = pos.bb(us, piece);
    let valid_pieces = pos.bb_for_color(!us);
    let empty_squares = !pos.bb_all();

    for (square, _) in pieces.iter() {
        let destinations = lookup.sliding_moves(square, pos.bb_all(), piece);
        let captures = destinations & valid_pieces;
        let quiets = destinations & empty_squares;

        extract_moves(square, captures, list, Capture);
        extract_moves(square, quiets, list, Quiet);
    }
}

pub fn gen_pseudo_legal_rook_moves(pos: &BoardState, list: &mut Vec<Move>, lookup: &Lookup) {
    let us = pos.active_player();
    let rooks = pos.bb(us, PieceType::Rook);
    let their_king = pos.bb(!us, PieceType::King);
    let valid_pieces = pos.bb_for_color(!us) & !their_king;
    let empty_squares = !pos.bb_all();

    for (square, _) in rooks.iter() {
        let destinations = lookup.sliding_moves(square, pos.bb_all(), PieceType::Rook);
        let captures = destinations & valid_pieces;
        let quiets = destinations & empty_squares;

        extract_moves(square, captures, list, Capture);
        extract_moves(square, quiets, list, Quiet);
    }
}

pub fn gen_pseudo_legal_bishop_moves(pos: &BoardState, list: &mut Vec<Move>, lookup: &Lookup) {
    let us = pos.active_player();
    let bishops = pos.bb(us, PieceType::Bishop);
    let their_king = pos.bb(!us, PieceType::King);
    let valid_pieces = pos.bb_for_color(!us) & !their_king;
    let empty_squares = !pos.bb_all();

    for (square, _) in bishops.iter() {
        let destinations = lookup.sliding_moves(square, pos.bb_all(), PieceType::Bishop);
        let captures = destinations & valid_pieces;
        let quiets = destinations & empty_squares;

        extract_moves(square, captures, list, Capture);
        extract_moves(square, quiets, list, Quiet);
    }
}

pub fn gen_pseudo_legal_queen_moves(pos: &BoardState, list: &mut Vec<Move>, lookup: &Lookup) {
    let us = pos.active_player();
    let queens = pos.bb(us, PieceType::Queen);
    let their_king = pos.bb(!us, PieceType::King);
    let valid_pieces = pos.bb_for_color(!us) & !their_king;
    let empty_squares = !pos.bb_all();

    for (square, _) in queens.iter() {
        let ray = lookup.sliding_moves(square, pos.bb_all(), PieceType::Queen);
        let captures = ray & valid_pieces;
        let quiets = ray & empty_squares;

        extract_moves(square, captures, list, Capture);
        extract_moves(square, quiets, list, Quiet);
    }
}
