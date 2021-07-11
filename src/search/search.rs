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
    let mv = Move::null();
    minimax(pos, mv, 3).mv
}

fn minimax(pos: &mut BoardState, mv: Move, depth: usize) -> MoveEval {
    if depth == 0 {
        return apply_and_eval(pos, mv);
    }

    return if pos.active_player() == Color::White {
        let mut max = MoveEval::min_null();
        for child in all_moves(pos).iter() {
            let mut new_pos = pos.clone();
            let eval = minimax(&mut new_pos, *child, depth - 1);
            max = MoveEval::max(max, eval);
        }
        max
    } else {
        let mut min = MoveEval::max_null();
        for child in all_moves(pos).iter() {
            let mut new_pos = pos.clone();
            let eval = minimax(&mut new_pos, *child, depth - 1);
            min = MoveEval::min(min, eval);
        }
        min
    };
}

fn apply_and_eval(pos: &BoardState, mv: Move) -> MoveEval {
    let mut p = pos.clone();
    p.make_move(mv);
    let eval = eval(&p);
    MoveEval { mv, eval }
}
