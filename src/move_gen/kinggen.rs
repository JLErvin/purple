use crate::board_state::board::BoardState;
use crate::components::bitboard::{Bitboard, ClearBit, PieceItr, Shift};
use crate::components::chess_move::{Move, MoveType, EAST, NORTH, SOUTH, WEST};
use crate::components::piece::PieceType;

use crate::components::chess_move::MoveType::{Capture, Quiet};
use crate::move_gen::lookup::Lookup;
use crate::move_gen::util::extract_moves;

pub fn gen_pseudo_legal_king_moves(pos: &BoardState, list: &mut Vec<Move>, lookup: &Lookup) {
    let us = pos.active_player();
    let king = pos.bb(us, PieceType::King);
    let empty_squares = !pos.bb_all();
    let enemies = pos.bb_for_color(!us);

    let square = king.trailing_zeros();
    let destinations = lookup.moves(square as u8, PieceType::King);
    let quiets = destinations & empty_squares;
    let captures = destinations & enemies;

    extract_moves(square as u8, captures, list, Capture);
    extract_moves(square as u8, quiets, list, Quiet);
}

#[cfg(test)]
mod test {
    use crate::board_state::fen::parse_fen;
    use crate::components::chess_move::Move;
    use crate::move_gen::kinggen::gen_pseudo_legal_king_moves;
    /*
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
    */
}
