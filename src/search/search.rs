use crate::board_state::board::BoardState;
use crate::components::chess_move::Move;
use crate::components::piece::Color;
use crate::move_gen::generator::all_moves;
use crate::search::eval::eval;
use core::mem;

pub fn best_move(pos: &mut BoardState) -> Move {
    let mut best_move = Box::new(Move::null());
    search(pos, 2, &mut best_move, true);
    *best_move
}

pub fn search(pos: &mut BoardState, depth: usize, best_move: &mut Box<Move>, set: bool) -> f64 {
    if depth == 0 {
        return eval(pos);
    }

    let moves = all_moves(pos);

    if pos.active_player() == Color::White {
        let mut max = f64::MIN;
        for child in moves.iter() {
            let mut new_pos = pos.clone();
            new_pos.make_move(*child);
            let mut eval = search(&mut new_pos, depth - 1, best_move, false);
            if eval > max {
                max = eval;
                if set {
                    mem::replace(best_move, Box::from(*child));
                }
            }
        }
        max
    } else {
        let mut min = f64::MAX;
        for child in moves.iter() {
            let mut new_pos = pos.clone();
            new_pos.make_move(*child);
            let mut eval = search(&mut new_pos, depth - 1, best_move, false);
            if eval < min {
                min = eval;
                if set {
                    mem::replace(best_move, Box::from(*child));
                }
            }
        }
        min
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
