use crate::board_state::board::BoardState;
use crate::components::bitboard::PieceItr;
use crate::components::piece::{Color, PieceType};
use crate::magic::util::MagicPiece::Rook;
use crate::move_gen::generator::all_moves;

const PAWN_VALUE: isize = 1;
const ROOK_VALUE: isize = 5;
const KNIGHT_VALUE: isize = 3;
const BISHOP_VALUE: isize = 3;
const KING_VALUE: isize = 200;
const QUEEN_VALUE: isize = 9;

const MOBILITY_VALUE: f32 = 0.1;

pub fn eval(pos: &BoardState) -> f64 {
    let moves = all_moves(pos);
    if moves.is_empty() {
        return if pos.active_player() == Color::White {
            f64::MIN
        } else {
            f64::MAX
        };
    }
    material_eval(pos) + mobility_eval(pos) + pawn_eval(pos)
}

#[inline]
fn material_eval(pos: &BoardState) -> f64 {
    let pawn_eval = piece_difference(pos, PieceType::Pawn) * PAWN_VALUE;
    let rook_eval = piece_difference(pos, PieceType::Rook) * ROOK_VALUE;
    let knight_eval = piece_difference(pos, PieceType::Knight) * KNIGHT_VALUE;
    let bishop_eval = piece_difference(pos, PieceType::Bishop) * BISHOP_VALUE;
    let queen_eval = piece_difference(pos, PieceType::Queen) * QUEEN_VALUE;
    let king_eval = piece_difference(pos, PieceType::King) * KING_VALUE;

    (pawn_eval + rook_eval + knight_eval + bishop_eval + queen_eval + king_eval) as f64
}

#[inline]
fn piece_difference(pos: &BoardState, piece: PieceType) -> isize {
    num_pieces(pos, Color::White, piece) - num_pieces(pos, Color::Black, piece)
}

#[inline]
fn num_pieces(pos: &BoardState, color: Color, piece: PieceType) -> isize {
    pos.bb(color, piece).iter().count() as isize
}

#[inline]
fn mobility_eval(pos: &BoardState) -> f64 {
    0.0
}

#[inline]
fn pawn_eval(pos: &BoardState) -> f64 {
    0.0
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board_state::fen::parse_fen;

    #[test]
    fn starting_position_equal_evaluation() {
        let pos = BoardState::default();
        let eval = eval(&pos);
        assert_eq!(eval, 0.0);
    }

    #[test]
    fn random_eval_1() {
        let pos =
            parse_fen(&"2b2R2/5pp1/3kPp2/2q5/Qr2PR2/8/Kp3P2/6N1 w - - 0 1".to_string()).unwrap();
        let eval = eval(&pos);
        assert_eq!(eval, 4.0);
    }
}
