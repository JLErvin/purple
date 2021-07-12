use crate::board_state::board::BoardState;
use crate::components::chess_move::Move;
use crate::components::piece::Color;
use crate::move_gen::generator::all_moves;
use crate::search::eval::eval;

/*func minimax(pos, depth, isMaximumPlayer):
if depth == 0 or game over
return evaluation of pos

if isMaximumPlayer
maxSoFar = -inf
for all children:
evaluation = minimax(child, depth - 1, false)
maxSoFar = max(maxSoFar, evaluation)
return maxSoFar
else
minSoFar = inf
for all children:
evaluation = minimax(child, depth - 1, true)
maxSoFar = min(minSoFar, evaluation)
return minSoFar
*/

#[derive(Copy, Clone)]
struct MoveEval {
    mv: Move,
    eval: f64,
}

impl MoveEval {
    fn min_null() -> MoveEval {
        let mv = Move::null();
        let eval = f64::MIN;
        MoveEval { mv, eval }
    }

    fn max_null() -> MoveEval {
        let mv = Move::null();
        let eval = f64::MAX;
        MoveEval { mv, eval }
    }

    fn max(mv1: MoveEval, mv2: MoveEval) -> MoveEval {
        if mv1.eval > mv2.eval {
            mv1
        } else {
            mv2
        }
    }

    fn min(mv1: MoveEval, mv2: MoveEval) -> MoveEval {
        if mv1.eval < mv2.eval {
            mv1
        } else {
            mv2
        }
    }
}

pub fn best_move(pos: &mut BoardState) -> Move {
    let moves = all_moves(pos);

    let mut best = if pos.active_player() == Color::White {
        MoveEval::min_null()
    } else {
        MoveEval::max_null()
    };
    for child in moves.iter() {
        let mut new_pos = pos.clone();
        new_pos.make_move(*child);
        let eval = minimax(&mut new_pos, 2);
        if pos.active_player() == Color::White {
            if eval > best.eval {
                best = MoveEval {
                    mv: *child,
                    eval: eval,
                }
            }
        } else {
            if eval < best.eval {
                best = MoveEval {
                    mv: *child,
                    eval: eval,
                }
            }
        }
    }
    best.mv
}

fn minimax(pos: &mut BoardState, depth: usize) -> f64 {
    if depth == 0 {
        return eval(pos);
    }

    return if pos.active_player() == Color::White {
        let mut max = MoveEval::min_null();
        let mut max = f64::MIN;
        for child in all_moves(pos).iter() {
            let mut new_pos = pos.clone();
            new_pos.make_move(*child);
            let mut eval = minimax(&mut new_pos, depth - 1);
            if eval > max {
                max = eval;
            }
        }
        max
    } else {
        let mut min = f64::MAX;
        for child in all_moves(pos).iter() {
            let mut new_pos = pos.clone();
            new_pos.make_move(*child);
            let mut eval = minimax(&mut new_pos, depth - 1);
            if eval < min {
                min = eval;
            }
        }
        min
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board_state::fen::parse_fen;

    #[test]
    fn finds_mate_in_one() {
        let mut pos = parse_fen(&"k7/8/2K5/8/8/8/8/1Q6 w - - 0 1".to_string()).unwrap();
        let mv = best_move(&mut pos);
        assert_eq!(mv.to, 49)
    }
}
