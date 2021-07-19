use crate::board_state::board::BoardState;
use crate::components::bitboard::{AddPiece, Bitboard, Shift};
use crate::components::chess_move::{Move, EAST, NORTH, SOUTH, WEST};
use crate::components::piece::{Color, PieceType};
use crate::components::square::Square;
use crate::magic::random::{GenerationScheme, MagicRandomizer};
use crate::move_gen::generator::MoveGenerator;
use crate::move_gen::lookup::Lookup;
use crate::search::eval::eval;
use core::mem;
use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::Neg;

#[derive(Eq, Copy, Clone, Debug)]
pub struct EvaledMove {
    mv: Move,
    eval: isize,
}

impl EvaledMove {
    pub fn null(eval: isize) -> EvaledMove {
        EvaledMove {
            mv: Move::null(),
            eval,
        }
    }
}

impl Ord for EvaledMove {
    fn cmp(&self, other: &EvaledMove) -> Ordering {
        self.eval.cmp(&other.eval)
    }
}

impl PartialOrd for EvaledMove {
    fn partial_cmp(&self, other: &EvaledMove) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EvaledMove {
    fn eq(&self, other: &EvaledMove) -> bool {
        self.eval == other.eval
    }
}

impl Neg for EvaledMove {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.eval = self.eval.wrapping_neg();
        self
    }
}

pub fn best_move(pos: &mut BoardState) -> Move {
    let gen = MoveGenerator::new();
    search(pos, &gen, 5).mv
}

fn simple_move(pos: &BoardState) -> EvaledMove {
    EvaledMove {
        mv: Move::null(),
        eval: eval(pos),
    }
}

pub fn search(pos: &mut BoardState, gen: &MoveGenerator, depth: usize) -> EvaledMove {
    if depth == 0 {
        return simple_move(pos);
    }

    let mut moves = gen
        .all_moves(pos)
        .into_iter()
        .map(|mv| EvaledMove { mv, eval: 0 })
        .collect_vec();

    let best = moves
        .into_iter()
        .map(|mut mv: EvaledMove| {
            let mut new_pos = pos.clone();
            new_pos.make_move(mv.mv);
            mv.eval = -search(&mut new_pos, gen, depth - 1).eval;
            mv
        })
        .max();

    return match best {
        None => handle_empty_moves(pos),
        Some(mv) => mv,
    };
}

fn handle_empty_moves(pos: &BoardState) -> EvaledMove {
    let random = MagicRandomizer::new(GenerationScheme::PreComputed);
    let lookup = Lookup::new(random);
    let is_in_check = is_attacked(pos, king_square(pos), &lookup);

    return if is_in_check {
        EvaledMove::null(-isize::MAX)
    } else {
        EvaledMove::null(0)
    };
}

pub fn king_square(pos: &BoardState) -> Square {
    let us = pos.active_player();
    pos.bb(us, PieceType::King).trailing_zeros() as Square
}

pub fn is_attacked(pos: &BoardState, square: Square, lookup: &Lookup) -> bool {
    let us = pos.active_player();

    if pawn_attacks(square, us) & pos.bb(!us, PieceType::Pawn) != 0 {
        return true;
    }

    let occupancies = pos.bb_all() & !pos.bb(us, PieceType::King);

    if lookup.sliding_moves(square, occupancies, PieceType::Rook)
        & (pos.bb(!us, PieceType::Rook) | pos.bb(!us, PieceType::Queen))
        != 0
    {
        return true;
    } else if lookup.sliding_moves(square, occupancies, PieceType::Bishop)
        & (pos.bb(!us, PieceType::Bishop) | pos.bb(!us, PieceType::Queen))
        != 0
    {
        return true;
    } else if lookup.moves(square, PieceType::Knight) & pos.bb(!us, PieceType::Knight) != 0 {
        return true;
    } else if lookup.moves(square, PieceType::King) & pos.bb(!us, PieceType::King) != 0 {
        return true;
    }

    false
}

pub fn pawn_attacks(square: Square, color: Color) -> Bitboard {
    let mut b: Bitboard = 0;
    let b = b.add_at_square(square);
    match color {
        Color::White => b.shift(NORTH + WEST) | b.shift(NORTH + EAST),
        Color::Black => b.shift(SOUTH + WEST) | b.shift(SOUTH + EAST),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board_state::fen::parse_fen;

    #[test]
    fn finds_mate_in_one() {
        let mut pos = parse_fen(&"k7/8/2K5/8/8/8/8/1Q6 w - - 0 1".to_string()).unwrap();
        let mv = best_move(&mut pos);
        println!("from: {} to: {}", mv.from, mv.to);
        assert_eq!(mv.to, 49)
    }
}
