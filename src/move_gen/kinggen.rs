use crate::board_state::board::BoardState;
use crate::components::bitboard::{Bitboard, ClearBit, Shift};
use crate::components::chess_move::{Move, MoveType, EAST, NORTH, SOUTH, WEST};
use crate::components::piece::PieceType;

pub fn gen_pseudo_legal_king_moves(pos: &BoardState, list: &mut Vec<Move>) {
    let us = pos.active_player();
    let king = pos.bb(us, PieceType::King);
    let empty_squares = !pos.bb_all();
    let enemies = pos.bb_for_color(!us);

    for direction in MoveType::king_itr() {
        let quiets = king.shift(*direction) & empty_squares;
        let captures = king.shift(*direction) & enemies;
        extract_moves(quiets, *direction, MoveType::Quiet, list);
        extract_moves(captures, *direction, MoveType::Capture, list);
    }
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
mod test {
    use crate::board_state::fen::parse_fen;
    use crate::components::chess_move::Move;
    use crate::move_gen::kinggen::gen_pseudo_legal_king_moves;

    #[test]
    fn moves_king_on_h1() {
        let pos = parse_fen(&"8/8/8/8/8/8/8/7K w - - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_king_moves(&pos, &mut list);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn moves_king_on_a1() {
        let pos = parse_fen(&"8/8/8/8/8/8/8/K7 w - - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_king_moves(&pos, &mut list);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn moves_king_on_a8() {
        let pos = parse_fen(&"K7/8/8/8/8/8/8/8 w - - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_king_moves(&pos, &mut list);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn moves_king_on_h8() {
        let pos = parse_fen(&"7K/8/8/8/8/8/8/8 w - - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_king_moves(&pos, &mut list);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn moves_king_random_fen1() {
        let pos =
            parse_fen(&"r4n2/4p1p1/5k1P/6pB/p7/1p1Pb1n1/2PB4/2K5 w - - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_king_moves(&pos, &mut list);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn moves_king_random_fen2() {
        let pos =
            parse_fen(&"6b1/P1P5/2B1P1p1/k1K2N1n/1p3N2/8/P2R3p/4b3 w - - 0 1".to_string()).unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        gen_pseudo_legal_king_moves(&pos, &mut list);
        assert_eq!(list.len(), 7);
    }
}
