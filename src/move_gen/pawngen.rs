use crate::board_state::board::BoardState;
use crate::components::bitboard::{AddPiece, Bitboard, ClearBit, PieceItr, Shift, RANK3, RANK7};
use crate::components::chess_move::MoveType::{
    BishopPromotion, BishopPromotionCapture, Capture, EnPassantCapture, KnightPromotion,
    KnightPromotionCapture, QueenPromotion, QueenPromotionCapture, Quiet, RookPromotion,
    RookPromotionCapture,
};
use crate::components::chess_move::{Move, MoveType, PromotionType, EAST, NORTH, WEST};
use crate::components::piece::PieceType;
use crate::components::piece::PieceType::{Knight, Queen};
use crate::move_gen::util::extract_moves;

/// Generate all pseudo-legal moves for the given position and add them
/// to the provided vector. Pseudo-legal moves are defined as a subset of
/// all legal moves for a given position which might also leave the king in check.
pub fn gen_pseudo_legal_pawn_moves(pos: &BoardState, list: &mut Vec<Move>) {
    gen_quiet_pushes(pos, list);
    gen_captures(pos, list);
    gen_en_passant(pos, list);
    gen_promotions(pos, list);
}

/// Generate all quiet pushes, defined as single and double pushes,
/// but excludes all promotions.
fn gen_quiet_pushes(pos: &BoardState, list: &mut Vec<Move>) {
    let us = pos.active_player();
    let pawns = pos.bb(us, PieceType::Pawn) & !RANK7;
    let empty_squares = !pos.bb_all();
    let single = pawns.shift(NORTH) & empty_squares;

    let pawns = single & RANK3;
    let empty_squares = !pos.bb_all();
    let double = pawns.shift(NORTH) & empty_squares;

    extract_moves(single, NORTH, Quiet, list);
    extract_moves(double, NORTH + NORTH, Quiet, list);
}

/// Generate all captures, including en-passant, but excluding captures which
/// result in promotions and under-promotions.
fn gen_captures(pos: &BoardState, list: &mut Vec<Move>) {
    let us = pos.active_player();
    let pawns = pos.bb(us, PieceType::Pawn) & !RANK7;
    let their_king = pos.bb(!us, PieceType::King);
    let valid_pieces = pos.bb_for_color(!us) & !their_king;

    let left_captures = pawns.shift(NORTH + WEST) & valid_pieces;
    let right_captures = pawns.shift(NORTH + EAST) & valid_pieces;

    extract_moves(left_captures, NORTH + WEST, Capture, list);
    extract_moves(right_captures, NORTH + EAST, Capture, list);
}

fn gen_en_passant(pos: &BoardState, list: &mut Vec<Move>) {
    if pos.en_passant().is_none() {
        return;
    }

    let us = pos.active_player();
    let en_passant = en_passant_bb(pos);
    let pawns = pos.bb(us, PieceType::Pawn);

    let left_captures = pawns.shift(NORTH + WEST) & en_passant;
    let right_captures = pawns.shift(NORTH + EAST) & en_passant;

    extract_moves(left_captures, NORTH + WEST, EnPassantCapture, list);
    extract_moves(right_captures, NORTH + EAST, EnPassantCapture, list);
}

/// Generate all promotions and under promotions, including pushes and captures on the eighth rank.
fn gen_promotions(pos: &BoardState, list: &mut Vec<Move>) {
    let us = pos.active_player();
    let pawns = pos.bb(us, PieceType::Pawn) & RANK7;
    let empty_squares = !pos.bb_all();
    let their_king = pos.bb(!us, PieceType::King);
    let valid_captures = pos.bb_for_color(!us) & !their_king;

    let pushes = pawns.shift(NORTH) & empty_squares;
    let left_captures = pawns.shift(NORTH + WEST) & valid_captures;
    let right_captures = pawns.shift(NORTH + EAST) & valid_captures;

    extract_promotions(pushes, NORTH, list, PromotionType::Push);
    extract_promotions(left_captures, NORTH + EAST, list, PromotionType::Capture);
    extract_promotions(right_captures, NORTH + WEST, list, PromotionType::Capture);
}

fn extract_promotions(
    mut bitboard: Bitboard,
    offset: i8,
    moves: &mut Vec<Move>,
    kind: PromotionType,
) {
    for (square, bb) in bitboard.iter() {
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

fn en_passant_bb(pos: &BoardState) -> Bitboard {
    let square = pos.en_passant().unwrap_or(0);
    if square == 0 {
        0
    } else {
        let b: Bitboard = 0;
        b.add_at_square(square)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_state::fen::parse_fen;
    use crate::components::bitboard::RANK2;
    use crate::components::square::SquareIndex::*;

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
        extract_moves(b, NORTH, Quiet, &mut moves);
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
        extract_moves(b, NORTH, Quiet, &mut moves);
        assert_eq!(moves.len(), 2);
        assert_eq!(moves.get(0).unwrap().to, D4 as u8);
        assert_eq!(moves.get(0).unwrap().from, D3 as u8);
        assert_eq!(moves.get(1).unwrap().to, F6 as u8);
        assert_eq!(moves.get(1).unwrap().from, F5 as u8);
    }
}
