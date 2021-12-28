use crate::board_state::board::BoardState;
use crate::common::bitboard::{
    AddPiece, Bitboard, New, PieceItr, Shift, RANK2, RANK3, RANK6, RANK7,
};
use crate::common::chess_move::MoveType::{
    Capture, EnPassantCapture, Quiet,
};
use crate::common::chess_move::{Move, MoveType, PromotionType, EAST, NORTH, SOUTH, WEST};

use crate::common::piece::{Color, PieceType};
use crate::common::square::Square;


#[derive(Copy, Clone)]
struct PawnDirections {
    rank7: Bitboard,
    rank3: Bitboard,
    north: i8,
    south: i8,
}

/// Generate all pseudo-legal moves for the given position and add them
/// to the provided vector. Pseudo-legal moves are defined as a subset of
/// all legal moves for a given position which might also leave the king in check.
pub fn gen_pseudo_legal_pawn_moves(pos: &BoardState, list: &mut Vec<Move>) {
    let dirs = PawnDirections::new(pos.active_player());
    let pawns = pos.bb(pos.active_player(), PieceType::Pawn);
    gen_quiet_pushes(pos, list, dirs, pawns);
    gen_captures(pos, list, dirs, pawns);
    gen_en_passant(pos, list, dirs, pawns);
    gen_promotions(pos, list, dirs, pawns);
}

/// Generate all quiet pushes, defined as single and double pushes,
/// but excludes all promotions.
fn gen_quiet_pushes(pos: &BoardState, list: &mut Vec<Move>, dirs: PawnDirections, pawns: Bitboard) {
    let pawns = pawns & !dirs.rank7;
    let empty_squares = !pos.bb_all();
    let single = pawns.shift(dirs.north) & empty_squares;

    let pawns = single & dirs.rank3;
    let empty_squares = !pos.bb_all();
    let double = pawns.shift(dirs.north) & empty_squares;

    extract_pawn_moves(single, dirs.north, Quiet, list);
    extract_pawn_moves(double, dirs.north + dirs.north, Quiet, list);
}

/// Generate all captures, excluding en passant captures and those which
/// result in promotions and under-promotions.
fn gen_captures(pos: &BoardState, list: &mut Vec<Move>, dirs: PawnDirections, pawns: Bitboard) {
    let us = pos.active_player();
    let pawns = pawns & !dirs.rank7;
    let their_king = pos.bb(!us, PieceType::King);
    let valid_pieces = pos.bb_for_color(!us) & !their_king;

    let left_captures = pawns.shift(dirs.north + WEST) & valid_pieces;
    let right_captures = pawns.shift(dirs.north + EAST) & valid_pieces;

    extract_pawn_moves(left_captures, dirs.north + WEST, Capture, list);
    extract_pawn_moves(right_captures, dirs.north + EAST, Capture, list);
}

/// Generate all en passant captures for the given position.
fn gen_en_passant(pos: &BoardState, list: &mut Vec<Move>, dirs: PawnDirections, pawns: Bitboard) {
    if pos.en_passant().is_none() {
        return;
    }

    let en_passant = en_passant_bb(pos);

    let left_captures = pawns.shift(dirs.north + WEST) & en_passant;
    let right_captures = pawns.shift(dirs.north + EAST) & en_passant;

    extract_pawn_moves(left_captures, dirs.north + WEST, EnPassantCapture, list);
    extract_pawn_moves(right_captures, dirs.north + EAST, EnPassantCapture, list);
}

/// Generate all promotions and under promotions, including pushes and captures on the eighth rank.
fn gen_promotions(pos: &BoardState, list: &mut Vec<Move>, dirs: PawnDirections, pawns: Bitboard) {
    let us = pos.active_player();
    let pawns = pawns & dirs.rank7;
    let empty_squares = !pos.bb_all();
    let their_king = pos.bb(!us, PieceType::King);
    let valid_captures = pos.bb_for_color(!us) & !their_king;

    let pushes = pawns.shift(dirs.north) & empty_squares;
    let left_captures = pawns.shift(dirs.north + WEST) & valid_captures;
    let right_captures = pawns.shift(dirs.north + EAST) & valid_captures;

    extract_promotions(pushes, dirs.north, list, PromotionType::Push);
    extract_promotions(
        left_captures,
        dirs.north + WEST,
        list,
        PromotionType::Capture,
    );
    extract_promotions(
        right_captures,
        dirs.north + EAST,
        list,
        PromotionType::Capture,
    );
}

/// Given a resulting bitboard and a relevant offset, find all pawn moves using the given offset.
pub fn extract_pawn_moves(bitboard: Bitboard, offset: i8, kind: MoveType, moves: &mut Vec<Move>) {
    for (square, _) in bitboard.iter() {
        let m = Move {
            to: square as u8,
            from: (square as i8 - offset) as u8,
            kind,
        };
        moves.push(m);
    }
}

/// Returns a bitboard representing all pawn attacks from the given square for the given color
pub fn pawn_attacks(square: Square, color: Color) -> Bitboard {
    let b: Bitboard = 0;
    let b = b.add_at_square(square);
    match color {
        Color::White => b.shift(NORTH + WEST) | b.shift(NORTH + EAST),
        Color::Black => b.shift(SOUTH + WEST) | b.shift(SOUTH + EAST),
    }
}

/// Given a resulting bitboard, find and enumerate all possible promotions using the provided offset.
fn extract_promotions(
    bitboard: Bitboard,
    offset: i8,
    moves: &mut Vec<Move>,
    kind: PromotionType,
) {
    for (square, _) in bitboard.iter() {
        let itr = match kind {
            PromotionType::Push => MoveType::promotion_itr(),
            PromotionType::Capture => MoveType::promotion_capture_itr(),
        };
        for promotion in itr {
            let m = Move {
                to: square as u8,
                from: (square as i8 - offset) as u8,
                kind: *promotion,
            };
            moves.push(m)
        }
    }
}

/// Given a game position, return a Bitboard that includes a non-zero bit only on the target en passant square.
fn en_passant_bb(pos: &BoardState) -> Bitboard {
    let square = pos.en_passant().unwrap_or(0);
    if square == 0 {
        0
    } else {
        Bitboard::for_square(square)
    }
}

impl PawnDirections {
    fn new(color: Color) -> PawnDirections {
        let rank7 = match color {
            Color::White => RANK7,
            Color::Black => RANK2,
        };
        let rank3 = match color {
            Color::White => RANK3,
            Color::Black => RANK6,
        };
        let north = match color {
            Color::White => NORTH,
            Color::Black => SOUTH,
        };
        PawnDirections {
            rank7,
            rank3,
            north,
            south: -north,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_state::fen::parse_fen;
    use crate::common::bitboard::RANK2;
    use crate::common::square::SquareIndex::*;

    #[test]
    fn gen_random_pawn_moves1() {
        let pos =
            parse_fen(&"3N4/1p1N2R1/kp3PQp/8/p2P4/B7/6p1/b2b2K1 w - - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn gen_random_pawn_moves2() {
        let pos =
            parse_fen(&"8/1P5n/1NB5/2KbQ1P1/2n5/p4R2/Pp2p3/1k2b3 w - - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn gen_random_pawn_moves3() {
        let pos =
            parse_fen(&"3r2r1/P6b/q2pKPk1/4P3/1p1P1R2/5n2/1B2N3/8 w - - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 7);
    }

    #[test]
    fn gen_random_pawn_moves4() {
        let pos = parse_fen(&"8/4PP2/2n3p1/6P1/2p1p2q/K1P3k1/b1p1P1B1/2R5 w - - 0 1".to_string())
            .unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 9);
    }

    #[test]
    fn gen_random_pawn_moves5() {
        let pos =
            parse_fen(&"3bBr2/8/P7/2PPp3/1N6/3bK2R/2Pp4/1k1qN3 w - d6 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 7);
    }

    #[test]
    fn extract_basic_pawn_moves() {
        let b = RANK2;
        let mut moves: Vec<Move> = Vec::new();
        extract_pawn_moves(b, NORTH, Quiet, &mut moves);
        assert_eq!(moves.len(), 8);
        assert_eq!(moves.get(0).unwrap().to, 8);
        assert_eq!(moves.get(1).unwrap().to, 9);
    }

    /// Pawns moves for FEN
    /// 3N4/1p1N2R1/kp3PQp/8/p2P4/B7/6p1/b2b2K1 w - - 0 1
    #[test]
    fn extract_random_pawns() {
        let b: Bitboard = 35184506306560;
        let mut moves: Vec<Move> = Vec::new();
        extract_pawn_moves(b, NORTH, Quiet, &mut moves);
        assert_eq!(moves.len(), 2);
        assert_eq!(moves.get(0).unwrap().to, D4 as u8);
        assert_eq!(moves.get(0).unwrap().from, D3 as u8);
        assert_eq!(moves.get(1).unwrap().to, F6 as u8);
        assert_eq!(moves.get(1).unwrap().from, F5 as u8);
    }

    #[test]
    fn gen_en_passant() {
        let pos = parse_fen(&"8/8/3p4/KPp4r/5R1k/8/8/8 w - c6 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn gen_a3_to_b4() {
        let pos = parse_fen(&"8/8/8/8/1p6/P7/8/8 w - - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn gen_b4_to_a3() {
        let mut pos = parse_fen(&"8/8/8/8/Pp6/8/8/8 b - a3 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
        let mv = *list.get(1).unwrap();
        pos.make_move(mv);
        assert_eq!(pos.bb_all(), 65536)
    }
}
