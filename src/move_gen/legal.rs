use crate::board_state::board::BoardState;
use crate::components::bitboard::{AddPiece, Bitboard, New, PieceItr};
use crate::components::chess_move::MoveType::Capture;
use crate::components::chess_move::{Move, MoveType, SOUTH};
use crate::components::piece::PieceType::Queen;
use crate::components::piece::{Color, PieceType};
use crate::components::square::Square;
use crate::move_gen::lookup::Lookup;
use crate::move_gen::pawns::pawn_attacks;
use crate::move_gen::util::knight_destinations;

/// Determines whether or not the given move is legal given the provided state of the game.
/// A move is determined to be legal if it does not leave the king in check after the move is made.
pub fn is_legal(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    let from = mv.from;

    let is_castle = mv.kind == MoveType::CastleKing || mv.kind == MoveType::CastleQueen;
    return if king_on_square(pos, from) & !is_castle {
        is_legal_king_move(pos, mv, lookup)
    } else {
        is_legal_non_king_move(pos, mv, lookup)
    };
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

    if mv.kind == MoveType::EnPassantCapture {
        return is_legal_en_passant(pos, mv, lookup);
    } else if mv.kind == MoveType::CastleKing || mv.kind == MoveType::CastleQueen {
        return is_legal_castle(pos, mv, lookup, num_checkers);
    }

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

/// Determines whether or not the given move is legal, working under the assumption that the provided
/// move represents a castling move. En Passant requires special checking since it is the only move in
/// which the piece moves to a square but does not capture on that square.
fn is_legal_en_passant(pos: &BoardState, mv: &Move, lookup: &Lookup) -> bool {
    let us = pos.active_player();
    let mut pos = pos.clone();

    let offset: i8 = match us {
        Color::White => 8,
        Color::Black => -8,
    };

    pos.remove_piece(PieceType::Pawn, !us, (mv.to as i8 - offset) as u8);
    let tmp_mv = Move {
        to: mv.to,
        from: mv.from,
        kind: Capture,
    };
    let is_legal = is_legal_non_king_move(&pos, &tmp_mv, lookup);
    pos.add(PieceType::Pawn, !us, (mv.to as i8 - offset) as u8);
    is_legal
}

/// Determines whether or not the given move is legal, working under the assumption that the given
/// move represents a castling move. A castle is illegal if the king is currently or would castle through a check.
fn is_legal_castle(pos: &BoardState, mv: &Move, lookup: &Lookup, num_checkers: u32) -> bool {
    if num_checkers != 0 {
        return false;
    }

    let squares: Vec<Square> = match mv.kind {
        MoveType::CastleKing => match pos.active_player() {
            Color::White => vec![5, 6],
            Color::Black => vec![61, 62],
        },
        MoveType::CastleQueen => match pos.active_player() {
            Color::White => vec![2, 3],
            Color::Black => vec![58, 59],
        },
        _ => vec![],
    };

    for square in squares.into_iter() {
        let attackers = attacks_to(pos, square, lookup);
        if attackers != 0 {
            return false;
        }
    }

    true
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
    let occupied = pos.bb_all();
    let piece_bb = Bitboard::for_square(mv.from);
    for piece in vec![PieceType::Rook, PieceType::Bishop, PieceType::Queen].into_iter() {
        let bb_itr = pos.bb(!us, piece);
        for (i, _) in bb_itr.iter() {
            /*            let attacks = lookup.sliding_moves(i, occupied, piece);
                        let reversed_attacks = lookup.sliding_moves(king_square(pos), occupied, piece);
                        let mask = lookup.between(king_square(pos), i);

                        let intersect = attacks & reversed_attacks & mask;
                        if intersect == 0 {
                            continue;
                        }

                        if intersect & pos.bb_for_color(us) != 0 {
                            return (true, i);
                        }
            */
            let attacks = lookup.sliding_moves(i, occupied, piece);
            //let mask = lookup.between(king_square(pos), i);
            let mask = ray_between(king_square(pos), i, lookup);
            let intersect = attacks & piece_bb & mask;
            if intersect == 0 {
                continue;
            }
            let removed = occupied & !piece_bb;
            let removed_attacks = lookup.sliding_moves(i, removed, piece);
            let king = pos.bb(us, PieceType::King);
            let intersect = removed_attacks & king & mask;
            if intersect != 0 {
                return (true, i);
            }
        }
    }
    (false, 0)
}

/// Returns the square where the active king currently sits (before the move is made).
fn king_square(pos: &BoardState) -> Square {
    let us = pos.active_player();
    pos.bb(us, PieceType::King).trailing_zeros() as Square
}

/// Returns a bitboard representing all pieces which are attacking the provided square.
fn attacks_to(pos: &BoardState, square: Square, lookup: &Lookup) -> Bitboard {
    let us = pos.active_player();
    let occupancies = pos.bb_all() & !pos.bb(us, PieceType::King);

    let pawn_attacks = pawn_attacks(square, us);
    let rook_attacks = lookup.sliding_moves(square, occupancies, PieceType::Rook);
    let bishop_attacks = lookup.sliding_moves(square, occupancies, PieceType::Bishop);
    let queen_attacks = rook_attacks | bishop_attacks;
    let knight_attacks = lookup.moves(square, PieceType::Knight);
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

    let king = pos.bb(pos.active_player(), PieceType::King);

    b & king != 0
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board_state::fen::parse_fen;
    use crate::components::chess_move::MoveType::Quiet;
    use crate::components::square::SquareIndex;
    use crate::components::square::SquareIndex::{
        A1, A2, A3, A4, B1, B2, B4, B8, C2, C3, C5, C6, C8, D3, D4, D5, E1, E2, E4, E6, E7, E8, F1,
        F2, F3, G1, G2, G5, G8, H1, H2, H3, H4,
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

    #[test]
    fn king_cannot_castle_through_check() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/8/8/8/8/3b4/8/R3K2R w KQ - 0 1".to_string()).unwrap();
        let mv = make_move(C2, D3);
        let mv = Move {
            to: 0,
            from: 0,
            kind: MoveType::CastleKing,
        };
        assert_eq!(is_legal_castle(&pos, &mv, &lookup, 0), false);
    }

    #[test]
    fn king_cannot_castle_in_check() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/8/8/8/8/2b5/8/R3K2R w KQ - 0 1".to_string()).unwrap();
        let mv = Move {
            to: 0,
            from: 0,
            kind: MoveType::CastleKing,
        };
        assert_eq!(is_legal_castle(&pos, &mv, &lookup, 1), false);
    }

    #[test]
    fn en_passant_discovered_check() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/8/8/K2Pp2q/8/8/8/8 w - e6 0 1".to_string()).unwrap();
        let mv = Move {
            to: E6 as u8,
            from: D5 as u8,
            kind: MoveType::EnPassantCapture,
        };
        assert_eq!(is_legal_en_passant(&pos, &mv, &lookup), false);
    }

    #[test]
    fn en_passant_out_of_check() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/8/8/3Pp2q/3K4/8/8/8 w - e6 0 1".to_string()).unwrap();
        let mv = Move {
            to: E6 as u8,
            from: D5 as u8,
            kind: MoveType::EnPassantCapture,
        };
        assert_eq!(is_legal_en_passant(&pos, &mv, &lookup), true);
    }

    #[test]
    fn random_fen_1() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/2p5/3p4/KP5r/5R1k/8/4P1P1/8 b - - 0 1".to_string()).unwrap();
        let mv = Move {
            to: G5 as u8,
            from: H4 as u8,
            kind: MoveType::Quiet,
        };
        assert_eq!(is_legal(&pos, &mv, &lookup), true);
    }

    #[test]
    fn random_fen_2() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(
            &"rnbqk1nr/pppp1ppp/8/4p3/1b1P4/P7/1PP1PPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        )
        .unwrap();
        let mv = Move {
            to: B4 as u8,
            from: A3 as u8,
            kind: MoveType::Capture,
        };
        assert_eq!(is_legal(&pos, &mv, &lookup), true);
    }

    #[test]
    fn random_fen_3() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(
            &"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/P1N2Q1p/1PPBBPPP/R3K2R w KQkq - 0 1".to_string(),
        )
        .unwrap();
        let mv = Move {
            to: A3 as u8,
            from: B4 as u8,
            kind: MoveType::Capture,
        };
        assert_eq!(is_legal(&pos, &mv, &lookup), true);
    }

    #[test]
    fn random_fen_4() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(
            &"r3k2r/p1ppqpb1/bn2pnp1/3PN3/Pp2P3/2N2Q1p/1PPBBPPP/R3K2R w KQkq a3 0 1".to_string(),
        )
        .unwrap();
        let mv = Move {
            to: A3 as u8,
            from: B4 as u8,
            kind: MoveType::EnPassantCapture,
        };
        assert_eq!(is_legal(&pos, &mv, &lookup), true);
    }

    #[test]
    fn castle_through_knight_attacks() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(
            &"r3k2r/p1ppqpb1/bnN1pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1".to_string(),
        )
        .unwrap();
        let mv = Move {
            to: C8 as u8,
            from: E8 as u8,
            kind: MoveType::CastleQueen,
        };
        assert_eq!(is_legal(&pos, &mv, &lookup), false);
    }

    #[test]
    fn castle_through_more_knight_attacks() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(
            &"r3k2r/p1ppqpb1/bn2pnN1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1".to_string(),
        )
        .unwrap();
        let mv = Move {
            to: G8 as u8,
            from: E8 as u8,
            kind: MoveType::CastleKing,
        };
        assert_eq!(is_legal(&pos, &mv, &lookup), false);
    }

    #[test]
    fn castle_through_even_more_knight_attacks() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(
            &"r3k2r/p1ppqNb1/bn2pn2/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1".to_string(),
        )
        .unwrap();
        let mv = Move {
            to: C8 as u8,
            from: E8 as u8,
            kind: MoveType::CastleQueen,
        };
        assert_eq!(is_legal(&pos, &mv, &lookup), false);
    }

    #[test]
    fn queen_captures() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(
            &"r3k2r/p1ppqpb1/1n2pnp1/3PN3/1p2P3/2N2Q1p/PPPBbPPP/R2K3R w KQkq - 0 1".to_string(),
        )
        .unwrap();
        let mv = Move {
            to: E2 as u8,
            from: F3 as u8,
            kind: MoveType::Capture,
        };
        assert_eq!(is_legal(&pos, &mv, &lookup), true);
    }

    #[test]
    fn challenge() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos =
            parse_fen(&"r6r/1bp2pP1/R2qkn2/1P6/1pPQ4/1B3N2/1B1P2p1/4K2R b K c3 0 1".to_string())
                .unwrap();
        let mv = Move {
            to: C3 as u8,
            from: B4 as u8,
            kind: MoveType::EnPassantCapture,
        };
        assert_eq!(is_legal(&pos, &mv, &lookup), false);
    }

    #[test]
    fn castle_pawn_attacks() {
        let random = MagicRandomizer::new(GenerationScheme::PreComputed);
        let lookup = Lookup::new(random);
        let pos = parse_fen(&"8/8/8/8/8/8/6p1/4K2R w K - 0 1".to_string()).unwrap();
        let mv = Move {
            to: E1 as u8,
            from: G1 as u8,
            kind: MoveType::CastleKing,
        };
        assert_eq!(is_legal(&pos, &mv, &lookup), false);
    }
}
