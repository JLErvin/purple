/*use crate::components::bitboard::{RANK7, Bitboard, GetBit, Shift};
use crate::components::piece::Piece::WPawn;
use crate::board_state::board::BoardState;

const MAX_MOVES: usize = 256;

pub fn gen_pawn_moves(pos: BoardState) -> Vec<Move> {
    let mut v: Vec<Move> = Vec::with_capacity(MAX_MOVES);
    v.append(&mut gen_single_pawn_moves(&pos));
    v.append(&mut gen_double_pawn_moves(&pos));
    v
}

fn gen_single_pawn_moves(pos: &BoardState) -> Vec<Move> {
    let pawns = pos.our_pawns() & !RANK7;
    let empty_squares = !pos.all();
    let forward = pawns.shift(UP) & empty_squares;
    extract_pawn_moves(forward)
}

fn extract_pawn_moves(b: Bitboard) -> Vec<Move> {
    let mut v: Vec<Move> = Vec::new();
    let mut i: i8 = 0;
    while b != 0 {
        let to = b.get_bit_lsb(0) as u8;
        if to != 0 {
            let m = Move {
                to: i,
                from: i - UP,
                piece: WPawn,
            };
            v.push(m);
        }
        i = i + 1;
    }
    v
}

fn gen_double_pawn_moves(pos: &GameState) -> Vec<Move> {
    let mut v: Vec<Move> = Vec::new();

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doesnt_crash() {}
}*/
