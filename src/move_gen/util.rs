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
