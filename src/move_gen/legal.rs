use crate::board_state::board::BoardState;
use crate::components::bitboard::{AddPiece, Bitboard, New, PieceItr};
use crate::components::chess_move::Move;
use crate::components::piece::{Color, PieceType};
use crate::components::square::Square;
use crate::move_gen::lookup::Lookup;
use crate::move_gen::pawns::pawn_attacks;
use crate::move_gen::util::knight_destinations;

/// Determines whether or not the given move is legal given the provided state of the game.
/// A move is determined to be legal if it does not leave the king in check after the move is made.
pub fn is_legal(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    let to = mv.to;
    let from = mv.from;

    let king_square = king_square(pos);
    let checkers = attacks_to(pos, king_square, lookup);
    let num_checkers = checkers.count_ones();

    if king_on_square(pos, from) {
        return is_legal_king_move(pos, mv, lookup);
    } else {
        return is_legal_non_king_move(pos, mv, lookup);
    }
}

/// Determines if the given move is legal, working under the assumption that the provided move
/// is a king move. Such a move is legal so long as the destination square of the king is not attacked
/// by the opponent's pieces.
fn is_legal_king_move(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    let to = mv.to;
    attacks_to(pos, to, lookup) == 0
}

fn is_legal_non_king_move(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    true
}

/// Returns the square where the active king currently sits (before the move is made).
fn king_square(pos: &BoardState) -> Square {
    let us = pos.active_player();
    pos.bb(us, PieceType::King).trailing_zeros() as Square
}

/// Returns a bitboard representing all pieces which are attacking the king.
fn attacks_to(pos: &BoardState, square: Square, lookup: &Lookup) -> Bitboard {
    let us = pos.active_player();
    let occupancies = pos.bb_all() & !pos.bb(us, PieceType::King);

    let pawn_attacks = pawn_attacks(square, us);
    let rook_attacks = lookup.sliding_moves(square, occupancies, PieceType::Rook);
    let bishop_attacks = lookup.sliding_moves(square, occupancies, PieceType::Bishop);
    let queen_attacks = rook_attacks | bishop_attacks;
    let knight_attacks = lookup.moves(square, PieceType::Bishop);
    let king_attacks = lookup.moves(square, PieceType::King);

    let pawns = pawn_attacks & pos.bb(!us, PieceType::Pawn);
    let rooks = rook_attacks & pos.bb(!us, PieceType::Rook);
    let bishops = bishop_attacks & pos.bb(!us, PieceType::Bishop);
    let queens = queen_attacks & pos.bb(!us, PieceType::Queen);
    let knights = knight_attacks & pos.bb(!us, PieceType::Knight);
    let king = king_attacks & pos.bb(!us, PieceType::King);

    pawns | rooks | bishops | queens | knights | king
}

pub fn cannot_move_because_pinned(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    // let's check against enemy rooks first
    let us = pos.active_player();
    let rooks = pos.bb(!us, PieceType::Rook);
    let occupied = pos.bb_all();
    let to = mv.to;
    let from = mv.from;
    let mut b: Bitboard = 0;
    b = b.add_at_square(from);

    if king_on_square(pos, from) {
        return false;
    }

    for (i, _) in rooks.iter() {
        let attacks = lookup.sliding_moves(i, occupied, PieceType::Rook);
        let intersect = attacks & b;
        if intersect == 0 {
            continue;
        }
        let removed = occupied & !b;
        let removed_attacks = lookup.sliding_moves(i, removed, PieceType::Rook);
        let king = pos.bb(us, PieceType::King);
        let intersect = removed_attacks & king;
        if intersect != 0 {
            // if this piece is pinned then the move is _only_ legal if we move on the ray that it's
            // pinned on.
            println!("Piece is pinned, cannot move to {}", to);
            let allowed_ray = ray_between(from, i, lookup);
            println!("Considering allowed_ray {}", allowed_ray);
            let mut b_to: Bitboard = 0;
            b_to = b_to.add_at_square(to);
            println!("Overlap with destination {}", b_to & allowed_ray);
            println!();

            return b_to & allowed_ray == 0;
        }
    }
    false
}

fn ray_between(s1: Square, s2: Square, lookup: &Lookup) -> Bitboard {
    let attack_s1 = lookup.sliding_moves(s1, 0, PieceType::Rook);
    let attack_s2 = lookup.sliding_moves(s2, 0, PieceType::Rook);

    let mut b1: Bitboard = 0;
    let mut b2: Bitboard = 0;
    b1 = b1.add_at_square(s1);
    b2 = b2.add_at_square(s2);

    (attack_s1 & attack_s2) | b1 | b2
}

pub fn is_legal_king_in_check(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    let us = pos.active_player();
    let to = mv.to;
    let from = mv.from;
    let occupancies = pos.bb_all();

    let our_king = pos.bb(us, PieceType::King);
    let king_square = our_king.trailing_zeros() as u8;

    let pawn_attacks = pawn_attacks(king_square, us);
    let rook_attacks = lookup.sliding_moves(king_square, occupancies, PieceType::Rook);
    let bishop_attacks = lookup.sliding_moves(king_square, occupancies, PieceType::Bishop);
    let queen_attacks = rook_attacks | bishop_attacks;
    let knight_attacks = knight_destinations(king_square);
    let king_attacks = lookup.moves(king_square, PieceType::King);

    let pawns = pawn_attacks & pos.bb(!us, PieceType::Pawn);
    let rooks = rook_attacks & pos.bb(!us, PieceType::Rook);
    let bishops = bishop_attacks & pos.bb(!us, PieceType::Bishop);
    let queens = queen_attacks & pos.bb(!us, PieceType::Queen);
    let knights = knight_attacks & pos.bb(!us, PieceType::Knight);
    let king = king_attacks & pos.bb(!us, PieceType::King);

    let is_king_under_attack = pawns | rooks | bishops | queens | knights | king != 0;

    !(is_king_under_attack && from != king_square)
}

pub fn is_legal_king(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    let us = pos.active_player();
    let to = mv.to;
    let from = mv.from;
    let occupancies = pos.bb_all() & !pos.bb(us, PieceType::King);

    if !king_on_square(pos, from) {
        return true;
    }

    let pawn_attacks = pawn_attacks(to, us);
    let rook_attacks = lookup.sliding_moves(to, occupancies, PieceType::Rook);
    let bishop_attacks = lookup.sliding_moves(to, occupancies, PieceType::Bishop);
    let queen_attacks = rook_attacks | bishop_attacks;
    let knight_attacks = knight_destinations(to);
    let king_attacks = lookup.moves(to, PieceType::King);

    let pawns = pawn_attacks & pos.bb(!us, PieceType::Pawn);
    let rooks = rook_attacks & pos.bb(!us, PieceType::Rook);
    let bishops = bishop_attacks & pos.bb(!us, PieceType::Bishop);
    let queens = queen_attacks & pos.bb(!us, PieceType::Queen);
    let knights = knight_attacks & pos.bb(!us, PieceType::Knight);
    let king = king_attacks & pos.bb(!us, PieceType::King);

    pawns | rooks | bishops | queens | knights | king == 0
}

fn king_on_square(pos: &BoardState, square: Square) -> bool {
    let b = Bitboard::for_square(square);

    let king = pos.bb(Color::White, PieceType::King);

    b & king != 0
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::board_state::fen::parse_fen;
    use crate::components::chess_move::MoveType::Quiet;
    use crate::components::square::SquareIndex::{A2, B1, B2};
    use crate::magic::random::{GenerationScheme, MagicRandomizer};

    #[test]
    fn legal_king_moves() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/8/8/8/8/8/1K5r/8 w - - 0 1".to_string()).unwrap();

        let mv = Move {
            to: A2 as u8,
            from: B2 as u8,
            kind: Quiet,
        };

        assert_eq!(is_legal_king_move(&pos, &mv, &lookup), false);

        let mv = Move {
            to: B1 as u8,
            from: B2 as u8,
            kind: Quiet,
        };

        assert_eq!(is_legal_king_move(&pos, &mv, &lookup), true);
    }
}
