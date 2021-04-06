use crate::board_state::board::BoardState;
use crate::components::bitboard::{AddPiece, Bitboard, PieceItr, FILEA, FILEB, FILEG, FILEH};
use crate::components::chess_move::MoveType::{Capture, Quiet};
use crate::components::chess_move::{Move, MoveType, EAST, NORTH, SOUTH, WEST};
use crate::components::piece::PieceType;
use crate::move_gen::lookup::Lookup;
use crate::move_gen::util::extract_moves;

pub fn gen_pseudo_legal_knight_moves(pos: &BoardState, list: &mut Vec<Move>, lookup: &Lookup) {
    let us = pos.active_player();
    let knights = pos.bb(us, PieceType::Knight);
    let valid_pieces = pos.bb_for_color(!us);
    let empty_squares = !pos.bb_all();

    for (square, _) in knights.iter() {
        let destinations = lookup.moves(square, PieceType::Knight);
        let captures = destinations & valid_pieces;
        let quiets = destinations & empty_squares;

        extract_moves(square as u8, captures, list, Capture);
        extract_moves(square as u8, quiets, list, Quiet);
    }
}

#[cfg(test)]
mod tests {
    use crate::board_state::fen::parse_fen;
    use crate::components::chess_move::Move;
    use crate::components::square::SquareIndex::{A1, A8, D4, H1, H8};
    use crate::move_gen::knightgen::gen_pseudo_legal_knight_moves;
    use crate::move_gen::util::knight_destinations;

    #[test]
    fn identifies_correct_attack_bb_from_a1() {
        let attack = knight_destinations(A1 as u8);
        assert_eq!(attack, 132096);
    }

    #[test]
    fn identifies_correct_attack_bb_from_a8() {
        let attack = knight_destinations(A8 as u8);
        assert_eq!(attack, 1128098930098176);
    }

    #[test]
    fn identifies_correct_attack_bb_from_h1() {
        let attack = knight_destinations(H1 as u8);
        assert_eq!(attack, 4202496);
    }

    #[test]
    fn identifies_correct_attack_bb_from_h8() {
        let attack = knight_destinations(H8 as u8);
        assert_eq!(attack, 9077567998918656);
    }

    #[test]
    fn identifies_correct_attack_bb_from_d4() {
        let attack = knight_destinations(D4 as u8);
        assert_eq!(attack, 22136263676928);
    }

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
