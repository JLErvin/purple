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
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 2, 3, 4, 4, 3, 2, 0, 0, 4, 6, 10, 10, 6,
    4, 0, 0, 6, 9, 10, 10, 9, 6, 0, 4, 8, 12, 16, 16, 12, 8, 4, 5, 10, 15, 20, 20, 15, 10, 5, 0, 0,
    0, 0, 0, 0, 0, 0,
];

const WHITE_KNIGHT_OPENING: [isize; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50, -40, -15, 0, 0, 0, 0, -15, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -20, 5, 15, 20, 20, 15, 5, -20, -20, 0, 15, 20, 20, 15, 0, -20, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -15, 0, 5, 5, 0, -15, -40, -50, -40, -30, -20, -20, -30, -40, -50,
];

const WHITE_BISHOP_OPENING: [isize; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

const WHITE_ROOK_OPENING: [isize; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];

const WHITE_QUEEN_OPENING: [isize; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

const WHITE_KING_OPENING: [isize; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

/// Given a given position, returns an estimated evaluation of the position based on a number of
/// hand-picked factors such as material difference, center control, tempo, pawn structure, etc.
/// Evaluations are determined to be relative to the active player.
pub fn eval(pos: &BoardState) -> isize {
    material_eval(pos)
        + mobility_eval(pos)
        + pawn_eval(pos)
        + rook_eval(pos)
        + knight_eval(pos)
        + bishop_eval(pos)
        + queen_eval(pos)
        + king_eval(pos)
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
    let mut white_score: isize = 0;
    let white_pawns = pos.bb(Color::White, PieceType::Pawn);
    for (square, _) in white_pawns.iter() {
        white_score += PAWN_ARRAY_WHITE[square as usize];
    }

    let mut black_score: isize = 0;
    let black_pawns = pos.bb(Color::Black, PieceType::Pawn);
    for (square, _) in black_pawns.iter() {
        black_score += PAWN_ARRAY_WHITE[63 - square as usize];
    }

    match pos.active_player {
        Color::Black => black_score - white_score,
        Color::White => white_score - black_score,
    }
}

#[inline]
fn rook_eval(pos: &BoardState) -> isize {
    let mut white_score: isize = 0;
    let white_pawns = pos.bb(Color::White, PieceType::Rook);
    for (square, _) in white_pawns.iter() {
        white_score += WHITE_ROOK_OPENING[square as usize];
    }

    let mut black_score: isize = 0;
    let black_pawns = pos.bb(Color::Black, PieceType::Rook);
    for (square, _) in black_pawns.iter() {
        black_score += WHITE_ROOK_OPENING[63 - square as usize];
    }

    match pos.active_player {
        Color::Black => black_score - white_score,
        Color::White => white_score - black_score,
    }
}

#[inline]
fn knight_eval(pos: &BoardState) -> isize {
    let mut white_score: isize = 0;
    let white_pawns = pos.bb(Color::White, PieceType::Knight);
    for (square, _) in white_pawns.iter() {
        white_score += WHITE_KNIGHT_OPENING[square as usize];
    }

    let mut black_score: isize = 0;
    let black_pawns = pos.bb(Color::Black, PieceType::Knight);
    for (square, _) in black_pawns.iter() {
        black_score += WHITE_KNIGHT_OPENING[63 - square as usize];
    }

    match pos.active_player {
        Color::Black => black_score - white_score,
        Color::White => white_score - black_score,
    }
}

#[inline]
fn bishop_eval(pos: &BoardState) -> isize {
    let mut white_score: isize = 0;
    let white_pawns = pos.bb(Color::White, PieceType::Bishop);
    for (square, _) in white_pawns.iter() {
        white_score += WHITE_BISHOP_OPENING[square as usize];
    }

    let mut black_score: isize = 0;
    let black_pawns = pos.bb(Color::Black, PieceType::Bishop);
    for (square, _) in black_pawns.iter() {
        black_score += WHITE_BISHOP_OPENING[63 - square as usize];
    }

    match pos.active_player {
        Color::Black => black_score - white_score,
        Color::White => white_score - black_score,
    }
}

#[inline]
fn queen_eval(pos: &BoardState) -> isize {
    let mut white_score: isize = 0;
    let white_pawns = pos.bb(Color::White, PieceType::Queen);
    for (square, _) in white_pawns.iter() {
        white_score += WHITE_QUEEN_OPENING[square as usize];
    }

    let mut black_score: isize = 0;
    let black_pawns = pos.bb(Color::Black, PieceType::Queen);
    for (square, _) in black_pawns.iter() {
        black_score += WHITE_QUEEN_OPENING[63 - square as usize];
    }

    match pos.active_player {
        Color::Black => black_score - white_score,
        Color::White => white_score - black_score,
    }
}

#[inline]
fn king_eval(pos: &BoardState) -> isize {
    let mut white_score: isize = 0;
    let white_pawns = pos.bb(Color::White, PieceType::King);
    for (square, _) in white_pawns.iter() {
        white_score += WHITE_KING_OPENING[square as usize];
    }

    let mut black_score: isize = 0;
    let black_pawns = pos.bb(Color::Black, PieceType::King);
    for (square, _) in black_pawns.iter() {
        black_score += WHITE_KING_OPENING[63 - square as usize];
    }

    match pos.active_player {
        Color::Black => black_score - white_score,
        Color::White => white_score - black_score,
    }
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
        let pos = parse_fen("2b2R2/5pp1/3kPp2/2q5/Qr2PR2/8/Kp3P2/6N1 w - - 0 1").unwrap();
        let eval = eval(&pos);
        assert!(eval < 400);
    }

    #[test]
    fn should_give_equal_evals_for_relative_color() {
        // Since the evaluation function is relative to the current player, flipping the player to move should give
        // the same evaluation in a symmetrical position
        let white_to_move_pos =
            parse_fen("2bqkbnr/pppppppp/4r3/3N4/3n4/4R3/PPPPPPPP/2BQKBNR w Kk - 0 1").unwrap();
        let black_to_move_pos =
            parse_fen("2bqkbnr/pppppppp/4r3/3N4/3n4/4R3/PPPPPPPP/2BQKBNR b Kk - 0 1").unwrap();

        let white_eval = eval(&white_to_move_pos);
        let black_eval = eval(&black_to_move_pos);

        assert_eq!(white_eval, black_eval);
    }
}
