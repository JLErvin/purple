use crate::bitboard::PieceItr;
use crate::board::BoardState;
use crate::piece::{Color, PieceType};

const PAWN_VALUE: isize = 100;
const ROOK_VALUE: isize = 500;
const KNIGHT_VALUE: isize = 300;
const BISHOP_VALUE: isize = 300;
const KING_VALUE: isize = 350;
const QUEEN_VALUE: isize = 800;

pub const MATE_VALUE: isize = 31_000;
pub const INF: isize = 32_001;
pub const NEG_INF: isize = -32_001;

const MOBILITY_VALUE: isize = 10;

const PAWN_ARRAY_WHITE: [isize; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, -5, -5, 0, 0, 0,
    0, 2, 3, 4, 4, 3, 2, 0,
    0, 4, 6, 10, 10, 6, 4, 0,
    0, 6, 9, 10, 10, 9, 6, 0,
    4, 8, 12, 16, 16, 12, 8, 4,
    5, 10, 15, 20, 20, 15, 10, 5,
    0, 0, 0, 0, 0, 0, 0, 0,
];

/// Given a given position, returns an estimated evaluation of the position based on a number of
/// hand-picked factors such as material difference, center control, tempo, pawn structure, etc.
/// Evaluations are determined to be relative to the active player.
pub fn eval(pos: &BoardState) -> isize {
    material_eval(pos) + mobility_eval(pos) + pawn_eval(pos)
}

#[inline]
fn material_eval(pos: &BoardState) -> isize {
    let pawn_eval = piece_difference(pos, PieceType::Pawn) * PAWN_VALUE;
    let rook_eval = piece_difference(pos, PieceType::Rook) * ROOK_VALUE;
    let knight_eval = piece_difference(pos, PieceType::Knight) * KNIGHT_VALUE;
    let bishop_eval = piece_difference(pos, PieceType::Bishop) * BISHOP_VALUE;
    let queen_eval = piece_difference(pos, PieceType::Queen) * QUEEN_VALUE;
    let king_eval = piece_difference(pos, PieceType::King) * KING_VALUE;

    pawn_eval + rook_eval + knight_eval + bishop_eval + queen_eval + king_eval
}

#[inline]
fn piece_difference(pos: &BoardState, piece: PieceType) -> isize {
    num_pieces(pos, pos.active_player, piece) - num_pieces(pos, !pos.active_player, piece)
}

#[inline]
fn num_pieces(pos: &BoardState, color: Color, piece: PieceType) -> isize {
    pos.bb(color, piece).iter().count() as isize
}

#[inline]
fn mobility_eval(_pos: &BoardState) -> isize {
    0 * MOBILITY_VALUE
}

#[inline]
fn pawn_eval(pos: &BoardState) -> isize {
    let mut our_score: isize = 0;
    let our_pawns = pos.bb(pos.active_player, PieceType::Pawn);

    for (square, _) in our_pawns.iter() {
        our_score += PAWN_ARRAY_WHITE[square as usize];
    }

    let mut their_score: isize = 0;
    let their_pawns = pos.bb(!pos.active_player, PieceType::Pawn);

    for (square, _) in their_pawns.iter() {
        their_score += PAWN_ARRAY_WHITE[64 - square as usize];
    }

    our_score - their_score
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fen::parse_fen;

    #[test]
    fn starting_position_equal_evaluation() {
        let pos = BoardState::default();
        let eval = eval(&pos);
        assert_eq!(eval, 0);
    }

    #[test]
    fn random_eval_1() {
        let pos =
            parse_fen(&"2b2R2/5pp1/3kPp2/2q5/Qr2PR2/8/Kp3P2/6N1 w - - 0 1".to_string()).unwrap();
        let eval = eval(&pos);
        assert_eq!(eval, 400);
    }
}
