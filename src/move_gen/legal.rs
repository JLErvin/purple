use crate::board_state::board::BoardState;
use crate::components::bitboard::{AddPiece, Bitboard};
use crate::components::chess_move::Move;
use crate::components::piece::{Color, PieceType};
use crate::components::square::Square;
use crate::move_gen::lookup::Lookup;
use crate::move_gen::pawns::pawn_attacks;
use crate::move_gen::util::knight_destinations;

pub fn is_legal(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    let us = pos.active_player();
    let to = mv.to;
    let from = mv.from;
    let occupancies = pos.bb_all();

    if !king_on_square(pos, from) {
        println!("King not on square {}", from);
        return true;
    }
    println!("King is on square {}", from);

    let pawn_attacks = pawn_attacks(to, us);
    let rook_attacks = lookup.sliding_moves(to, occupancies, PieceType::Rook);
    let bishop_attacks = lookup.sliding_moves(to, occupancies, PieceType::Bishop);
    let queen_attacks = rook_attacks | bishop_attacks;
    let knight_attacks = knight_destinations(to);
    let king_attacks = lookup.moves(to, PieceType::King);

    println!("Pawn attacks: {}", pawn_attacks);
    println!("Rook attacks: {}", pawn_attacks);
    println!("Bishop attacks: {}", pawn_attacks);
    println!("Queen attacks: {}", pawn_attacks);
    println!("Knight attacks: {}", pawn_attacks);
    println!("King attacks: {}", pawn_attacks);

    let pawns = pawn_attacks & pos.bb(!us, PieceType::Pawn);
    let rooks = rook_attacks & pos.bb(!us, PieceType::Rook);
    let bishops = bishop_attacks & pos.bb(!us, PieceType::Bishop);
    let queens = queen_attacks & pos.bb(!us, PieceType::Queen);
    let knights = knight_attacks & pos.bb(!us, PieceType::Knight);
    let king = king_attacks & pos.bb(!us, PieceType::King);

    pawns | rooks | bishops | queens | knights | king == 0
}

fn king_on_square(pos: &BoardState, square: Square) -> bool {
    let mut b: Bitboard = 0;
    b = b.add_at_square(square);

    let king = pos.bb(Color::White, PieceType::King);

    b & king != 0
}
