use crate::components::chess_move::Move;
use std::cmp::Ordering;
use std::ops::Neg;

#[derive(Eq, Copy, Clone, Debug)]
pub struct EvaledMove {
    pub mv: Move,
    pub eval: isize,
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
