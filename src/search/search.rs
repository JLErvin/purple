use crate::board_state::board::BoardState;
use crate::components::bitboard::{AddPiece, Bitboard, Shift};
use crate::components::chess_move::{Move, EAST, NORTH, SOUTH, WEST};
use crate::components::piece::{Color, PieceType};
use crate::magic::random::{GenerationScheme, MagicRandomizer};
use crate::move_gen::generator::MoveGenerator;
use crate::move_gen::lookup::Lookup;
use crate::move_gen::util::{is_attacked, king_square};
use crate::search::eval::{eval, INF, MATE_VALUE, NEG_INF};
use crate::search::eval_move::EvaledMove;
use itertools::Itertools;

pub fn best_move(pos: &mut BoardState) -> Move {
    let gen = MoveGenerator::new();
    //minimax(pos, &gen, 5).mv
    alpha_beta(pos, &gen, NEG_INF, INF, 5).mv
}

pub fn alpha_beta(
    pos: &mut BoardState,
    gen: &MoveGenerator,
    mut alpha: isize,
    mut beta: isize,
    depth: usize,
) -> EvaledMove {
    if depth == 0 {
        return EvaledMove::null(eval(pos));
    }

    let mut moves = gen
        .all_moves(pos)
        .into_iter()
        .map(|mv| EvaledMove { mv, eval: 0 })
        .collect_vec();

    if moves.is_empty() {
        return handle_empty_moves(pos, depth);
    }

    let mut best_move = EvaledMove::null(alpha);
    for mov in moves.iter_mut() {
        let mut new_pos = pos.clone_with_move(mov.mv);
        mov.eval = -alpha_beta(&mut new_pos, gen, -beta, -alpha, depth - 1).eval;
        if mov.eval > alpha {
            alpha = mov.eval;
            if alpha >= beta {
                return *mov;
            }
            best_move = *mov;
        }
    }

    best_move
}

pub fn minimax(pos: &mut BoardState, gen: &MoveGenerator, depth: usize) -> EvaledMove {
    if depth == 0 {
        return EvaledMove::null(eval(pos));
    }

    let moves = gen
        .all_moves(pos)
        .into_iter()
        .map(|mv| EvaledMove { mv, eval: 0 })
        .collect_vec();

    let best = moves
        .into_iter()
        .map(|mut mv: EvaledMove| {
            let mut new_pos = pos.clone_with_move(mv.mv);
            mv.eval = -minimax(&mut new_pos, gen, depth - 1).eval;
            mv
        })
        .max();

    match best {
        None => handle_empty_moves(pos, depth),
        Some(mv) => mv,
    }
}

fn handle_empty_moves(pos: &BoardState, depth: usize) -> EvaledMove {
    let random = MagicRandomizer::new(GenerationScheme::PreComputed);
    let lookup = Lookup::new(random);
    let is_in_check = is_attacked(pos, king_square(pos), &lookup);

    if is_in_check {
        EvaledMove::null(-MATE_VALUE - depth as isize)
    } else {
        EvaledMove::null(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board_state::fen::parse_fen;

    #[test]
    fn finds_mate_in_one_as_white() {
        let mut pos = parse_fen(&"k7/8/2K5/8/8/8/8/1Q6 w - - 0 1".to_string()).unwrap();
        let mv = best_move(&mut pos);
        println!("from: {} to: {}", mv.from, mv.to);
    }

    #[test]
    fn finds_mate_in_one_as_black() {
        let mut pos = parse_fen(&"K7/8/2k5/8/8/8/8/1q6 b - - 0 1".to_string()).unwrap();
        let mv = best_move(&mut pos);
        println!("from: {} to: {}", mv.from, mv.to);
        assert_eq!(mv.to, 49)
    }
}
