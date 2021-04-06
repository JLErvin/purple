use crate::board_state::board::BoardState;
use crate::board_state::position::Position;
use crate::components::bitboard::{Bitboard, PieceItr};
use crate::components::chess_move::MoveType::{Capture, Quiet};
use crate::components::chess_move::{Move, MoveType};
use crate::components::piece::PieceType;
use crate::move_gen::lookup::Lookup;
use crate::move_gen::util::extract_moves;
use std::io::empty;

pub fn gen_pseudo_legal_moves(
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
        let destinations = match piece {
            PieceType::King | PieceType::Knight => lookup.moves(square, piece),
            _ => lookup.sliding_moves(square, pos.bb_all(), piece),
        };
        let captures = destinations & valid_pieces;
        let quiets = destinations & empty_squares;

        extract_moves(square, captures, list, Capture);
        extract_moves(square, quiets, list, Quiet);
    }
}

#[cfg(test)]
mod test {
    use crate::board_state::fen::parse_fen;
    use crate::components::chess_move::Move;
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

    /*    #[test]
       fn identifies_number_of_moves_from_d4() {
           let pos = parse_fen(&"8/8/8/8/3N4/8/8/8 w - - 0 1".to_string()).unwrap();
           let mut list: Vec<Move> = Vec::new();
           gen_pseudo_legal_knight_moves(&pos, &mut list);
           assert_eq!(list.len(), 8);
       }

       #[test]
       fn identifies_number_of_moves_from_h1() {
           let pos = parse_fen(&"8/8/8/8/8/8/8/7N w - - 0 1".to_string()).unwrap();
           let mut list: Vec<Move> = Vec::new();
           gen_pseudo_legal_knight_moves(&pos, &mut list);
           assert_eq!(list.len(), 2);
       }

       #[test]
       fn identifies_number_of_moves_from_random_fen1() {
           let pos =
               parse_fen(&"R7/2p5/1p4PK/1PpN1PP1/4PP2/6p1/2n4k/2B5 w - - 0 1".to_string()).unwrap();
           let mut list: Vec<Move> = Vec::new();
           gen_pseudo_legal_knight_moves(&pos, &mut list);
           assert_eq!(list.len(), 7);
       }

       #[test]
       fn identifies_number_of_moves_from_random_fen2() {
           let pos =
               parse_fen(&"7n/QpB1N3/8/1p6/1p1b1R1P/p1PN4/4Kp2/1k6 w - - 0 1".to_string()).unwrap();
           let mut list: Vec<Move> = Vec::new();
           gen_pseudo_legal_knight_moves(&pos, &mut list);
           assert_eq!(list.len(), 13);
       }

    */
}
