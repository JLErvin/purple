use crate::board_state::board::BoardState;
use crate::components::bitboard::{AddPiece, Bitboard, ClearBit, Shift, RANK3, RANK7};
use crate::components::chess_move::MoveType::{
    BishopPromotion, BishopPromotionCapture, Capture, KnightPromotion, KnightPromotionCapture,
    QueenPromotion, QueenPromotionCapture, Quiet, RookPromotion, RookPromotionCapture,
};
use crate::components::chess_move::{Move, MoveType, EAST, NORTH, WEST};
use crate::components::piece::PieceType;
use crate::components::piece::PieceType::{Knight, Queen};

pub fn gen_pseudo_legal_pawn_moves(pos: &BoardState, list: &mut Vec<Move>) {
    gen_quiet_pushes(pos, list);
    gen_en_passant(pos, list);
    gen_captures(pos, list);
    gen_promotions(pos, list);
}

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

fn gen_en_passant(pos: &BoardState, list: &mut Vec<Move>) {
    if pos.en_passant().is_none() {
        return;
    }

    let us = pos.active_player();
    let square = pos.en_passant().unwrap();
    let mut b: Bitboard = 0;
    b = b.add_at_square(square);
    let pawns = pos.bb(us, PieceType::Pawn);

    let left_captures = pawns.shift(NORTH + WEST) & b;
    let right_captures = pawns.shift(NORTH + EAST) & b;

    extract_moves(left_captures, NORTH + WEST, Capture, list);
    extract_moves(right_captures, NORTH + EAST, Capture, list);
}

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

fn gen_promotions(pos: &BoardState, list: &mut Vec<Move>) {
    let us = pos.active_player();
    let pawns = pos.bb(us, PieceType::Pawn) & RANK7;
    let empty_squares = !pos.bb_all();
    let their_king = pos.bb(!us, PieceType::King);
    let valid_captures = pos.bb_for_color(!us) & !their_king;

    let pushes = pawns.shift(NORTH) & empty_squares;
    let left_captures = pawns.shift(NORTH + WEST) & valid_captures;
    let right_captures = pawns.shift(NORTH + EAST) & valid_captures;

    extract_moves(pushes, NORTH, KnightPromotion, list);
    extract_moves(pushes, NORTH, BishopPromotion, list);
    extract_moves(pushes, NORTH, RookPromotion, list);
    extract_moves(pushes, NORTH, QueenPromotion, list);

    extract_moves(left_captures, NORTH + EAST, KnightPromotionCapture, list);
    extract_moves(left_captures, NORTH + EAST, BishopPromotionCapture, list);
    extract_moves(left_captures, NORTH + EAST, RookPromotionCapture, list);
    extract_moves(left_captures, NORTH + EAST, QueenPromotionCapture, list);

    extract_moves(right_captures, NORTH + WEST, KnightPromotionCapture, list);
    extract_moves(right_captures, NORTH + WEST, BishopPromotionCapture, list);
    extract_moves(right_captures, NORTH + WEST, RookPromotionCapture, list);
    extract_moves(right_captures, NORTH + WEST, QueenPromotionCapture, list);
}

fn extract_moves(mut bitboard: Bitboard, offset: i8, kind: MoveType, moves: &mut Vec<Move>) {
    while bitboard != 0 {
        let index = bitboard.trailing_zeros() as u8;
        bitboard = bitboard.clear_bit(index);
        let m = Move {
            to: index as u8,
            from: (index as i8 - offset) as u8,
            kind,
        };
        moves.push(m);
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
        let n: u64 = 281474976710656;
        let k = n.checked_shl(9).unwrap_or(0);

        let pos =
            parse_fen(&"3r2r1/P6b/q2pKPk1/4P3/1p1P1R2/5n2/1B2N3/8 w - - 0 1".to_string()).unwrap();
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
