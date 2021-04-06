use crate::components::bitboard::{AddPiece, PieceItr, FILEA, FILEB, FILEG, FILEH};
use crate::components::chess_move::{Move, EAST, NORTH, SOUTH, WEST};
use crate::components::{bitboard::Bitboard, chess_move::MoveType};

pub fn extract_moves(from: u8, bb: Bitboard, list: &mut Vec<Move>, kind: MoveType) {
    for (square, bb) in bb.iter() {
        let m = Move {
            to: square,
            from,
            kind,
        };
        list.push(m);
    }
}

pub fn knight_destinations(square: u8) -> Bitboard {
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
}
