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

/// Determines if the given move is legal, working under the assumption that the provided move
/// is not a king move. Such a move is legal under the following conditions:
/// 1. If the king is attacked by more than once piece, the move will always be illegal
/// 2. If the king is attacked by one piece, the move is legal iff we block the attack or capture
///    the attacking piece and do not expose the king in the process.
/// 3. If the given piece is pinned the move is legal only if we move along the pinned ray or capture
///    the attacking piece.
/// 4. If the king is not attacked and the piece is not pinned the move will always be legal.
fn is_legal_non_king_move(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    let king_square = king_square(pos);
    let checkers = attacks_to(pos, king_square, lookup);
    let num_checkers = checkers.count_ones();

    // If more than one piece has put the king in check then the only legal move is for the king to move
    // and evade checks - hence a non-king move will always be illegal.
    if num_checkers > 1 {
        return false;
    }

    let pinned = is_absolutely_pinned(pos, mv, lookup);

    // If exactly one piece puts us in check then our move is legal iff we block the incoming attack
    // or we capture the attacking piece.
    if num_checkers == 1 {
        let piece_bb = Bitboard::for_square(mv.to);
        let attacker_square = checkers.trailing_zeros() as u8;
        return if mv.to == attacker_square {
            !pinned.0
        } else {
            let attacking_ray = ray_between(king_square, attacker_square, lookup);
            !pinned.0 && (attacking_ray & piece_bb != 0)
        };
    }

    // If a piece is not absolutely pinned then it is free to move anywhere since we have already
    // determined the king is not currently in check.
    if !pinned.0 {
        return true;
    }

    return is_legal_pin_move(pos, mv, lookup, pinned.1);
}

/// Determines whether or not the given move is legal, working under the assumption that the moved
/// piece is currently pinned. Such a move is legal iff we move along the pinning ray or we caputre
/// the attacking piece
fn is_legal_pin_move(pos: &BoardState, mv: &Move, lookup: &Lookup, pinner: Square) -> bool {
    let king_square = king_square(pos);
    let piece_bb = Bitboard::for_square(mv.to);
    let attack_ray = ray_between(king_square, pinner, lookup);

    attack_ray & piece_bb != 0
}

/// Determines whether or not the given piece being moved is pinned. If the piece is pinned, the returned Square
/// represents the square of the pinning piece.
fn is_absolutely_pinned(pos: &BoardState, mv: &Move, lookup: &Lookup) -> (bool, Square) {
    let us = pos.active_player();
    let rooks = pos.bb(!us, PieceType::Rook);
    let occupied = pos.bb_all();
    let piece_bb = Bitboard::for_square(mv.from);
    for (i, _) in rooks.iter() {
        let attacks = lookup.sliding_moves(i, occupied, PieceType::Rook);
        let intersect = attacks & piece_bb;
        if intersect == 0 {
            continue;
        }
        let removed = occupied & !piece_bb;
        let removed_attacks = lookup.sliding_moves(i, removed, PieceType::Rook);
        let king = pos.bb(us, PieceType::King);
        let intersect = removed_attacks & king;
        if intersect != 0 {
            return (true, i);
        }
    }
    (false, 0)
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

fn ray_between(s1: Square, s2: Square, lookup: &Lookup) -> Bitboard {
    let full: Bitboard = !0;
    let b1 = Bitboard::for_square(s1);
    let b2 = Bitboard::for_square(s2);
    lookup.between(s1, s2) & ((full << s1) ^ (full << s2)) | b1 | b2
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
    use crate::components::square::SquareIndex;
    use crate::components::square::SquareIndex::{
        A1, A2, B1, B2, B4, B8, C2, C5, C6, D3, D4, E2, E7, F1, F2, G1, G2, H1, H2,
    };
    use crate::magic::random::{GenerationScheme, MagicRandomizer};

    fn make_move(to: SquareIndex, from: SquareIndex) -> Move {
        Move {
            to: to as u8,
            from: from as u8,
            kind: Quiet,
        }
    }

    #[test]
    fn moves_between_same_rank() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let b = ray_between(A1 as u8, H1 as u8, &lookup);

        assert_eq!(b, 255);
    }

    #[test]
    fn moves_along_diagonal() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let b = ray_between(B4 as u8, E7 as u8, &lookup);

        assert_eq!(b, 4512412933816320);
    }

    #[test]
    fn cannot_capture_checking_piece_while_pinned() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"2r5/8/8/2B5/8/8/8/2K3r1 w - - 0 1".to_string()).unwrap();

        let mv = make_move(G1, C5);
        assert_eq!(is_legal_non_king_move(&pos, &mv, &lookup), false);
    }

    #[test]
    fn cannot_block_checking_piece_while_pinned() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"2r5/8/8/2B5/8/8/8/2K4r w - - 0 1".to_string()).unwrap();

        let mv = make_move(G1, C5);
        assert_eq!(is_legal_non_king_move(&pos, &mv, &lookup), false);
    }

    #[test]
    fn cannot_move_pinned_piece() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/8/8/8/1K1N3r/8/8/8 w - - 0 1".to_string()).unwrap();

        let mv = make_move(C6, D4);
        assert_eq!(is_legal_non_king_move(&pos, &mv, &lookup), false);

        let mv = make_move(C2, D4);
        assert_eq!(is_legal_non_king_move(&pos, &mv, &lookup), false);
    }

    #[test]
    fn can_move_piece_along_pinned_ray() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/8/8/8/8/8/1K3R1r/8 w - - 0 1".to_string()).unwrap();

        // Move towards pinner without capture
        let mv = make_move(G2, F2);
        assert_eq!(is_legal_non_king_move(&pos, &mv, &lookup), true);

        // Move towards pinner with capture
        let mv = make_move(H2, F2);
        assert_eq!(is_legal_non_king_move(&pos, &mv, &lookup), true);

        // Move away from pinner
        let mv = make_move(E2, F2);
        assert_eq!(is_legal_non_king_move(&pos, &mv, &lookup), true);

        // Moving off pin is illegal
        let mv = make_move(F1, F2);
        assert_eq!(is_legal_non_king_move(&pos, &mv, &lookup), false);
    }

    #[test]
    fn cannot_move_non_king_with_multiple_checkers() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/1r6/8/8/3N4/8/1K5r/8 w - - 0 1".to_string()).unwrap();
        let mv = make_move(D4, C6);
        assert_eq!(is_legal_non_king_move(&pos, &mv, &lookup), false);
    }

    #[test]
    fn can_move_king() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/8/8/8/8/8/1K5r/8 w - - 0 1".to_string()).unwrap();
        let mv = make_move(A2, B2);
        assert_eq!(is_legal_king_move(&pos, &mv, &lookup), false);
        let mv = make_move(B1, B2);
        assert_eq!(is_legal_king_move(&pos, &mv, &lookup), true);
    }

    #[test]
    fn cannot_block_using_xray() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/8/8/8/8/3B4/3K3r/8 w - - 0 1".to_string()).unwrap();
        let mv = make_move(C2, D3);
        assert_eq!(is_legal_non_king_move(&pos, &mv, &lookup), false);
        let mv = make_move(E2, D3);
        assert_eq!(is_legal_non_king_move(&pos, &mv, &lookup), true);
    }
}
