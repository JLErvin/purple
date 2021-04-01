use crate::board_state::board::BoardState;
use crate::components::bitboard::{AddPiece, Bitboard, PieceItr, FILEA, FILEB, FILEG, FILEH};
use crate::components::chess_move::MoveType::{Capture, Quiet};
use crate::components::chess_move::{Move, MoveType, EAST, NORTH, SOUTH, WEST};
use crate::components::piece::PieceType;

pub fn gen_pseudo_legal_knight_moves(pos: &BoardState, list: &mut Vec<Move>) {
    let us = pos.active_player();
    let knights = pos.bb(us, PieceType::Knight);
    let their_king = pos.bb(!us, PieceType::King);
    let valid_pieces = pos.bb_for_color(!us) & !their_king;
    let empty_squares = !pos.bb_all();

    for (square, bb) in knights.iter() {
        let destinations = attacks_from_square(square);
        let captures = destinations & valid_pieces;
        let quiets = destinations & empty_squares;

        extract_moves(square, captures, list, Capture);
        extract_moves(square, quiets, list, Quiet);
    }
}

fn extract_moves(from: u8, bb: Bitboard, list: &mut Vec<Move>, kind: MoveType) {
    for (square, bb) in bb.iter() {
        let m = Move {
            to: square,
            from,
            kind,
        };
        list.push(m);
    }
}

fn attacks_from_square(square: u8) -> Bitboard {
    let base_bb: Bitboard = 0;
    let base_bb = base_bb.add_at_square(square);

    let nnw = base_bb
        .checked_shl((NORTH + NORTH + WEST) as u32)
        .unwrap_or(0)
        & !FILEH;
    let nww = base_bb
        .checked_shl((NORTH + WEST + WEST) as u32)
        .unwrap_or(0)
        & !(FILEH | FILEG);
    let nne = base_bb
        .checked_shl((NORTH + NORTH + EAST) as u32)
        .unwrap_or(0)
        & !FILEA;
    let nee = base_bb
        .checked_shl((NORTH + EAST + EAST) as u32)
        .unwrap_or(0)
        & !(FILEA | FILEB);

    let sww = base_bb
        .checked_shr(-(SOUTH + WEST + WEST) as u32)
        .unwrap_or(0)
        & !(FILEG | FILEH);
    let ssw = base_bb
        .checked_shr(-(SOUTH + SOUTH + WEST) as u32)
        .unwrap_or(0)
        & !FILEH;
    let sse = base_bb
        .checked_shr(-(SOUTH + SOUTH + EAST) as u32)
        .unwrap_or(0)
        & !FILEA;
    let see = base_bb
        .checked_shr(-(SOUTH + EAST + EAST) as u32)
        .unwrap_or(0)
        & !(FILEA | FILEB);

    nnw | nww | nne | nee | sww | ssw | sse | see
}

#[cfg(test)]
mod tests {
    use crate::board_state::fen::parse_fen;
    use crate::components::chess_move::Move;
    use crate::components::square::SquareIndex::{A1, A8, D4, H1, H8};
    use crate::move_gen::knightgen::{attacks_from_square, gen_pseudo_legal_knight_moves};

    #[test]
    fn identifies_correct_attack_bb_from_a1() {
        let attack = attacks_from_square(A1 as u8);
        assert_eq!(attack, 132096);
    }

    #[test]
    fn identifies_correct_attack_bb_from_a8() {
        let attack = attacks_from_square(A8 as u8);
        assert_eq!(attack, 1128098930098176);
    }

    #[test]
    fn identifies_correct_attack_bb_from_h1() {
        let attack = attacks_from_square(H1 as u8);
        assert_eq!(attack, 4202496);
    }

    #[test]
    fn identifies_correct_attack_bb_from_h8() {
        let attack = attacks_from_square(H8 as u8);
        assert_eq!(attack, 9077567998918656);
    }

    #[test]
    fn identifies_correct_attack_bb_from_d4() {
        let attack = attacks_from_square(D4 as u8);
        assert_eq!(attack, 22136263676928);
    }

    #[test]
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
}
