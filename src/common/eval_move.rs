use crate::common::chess_move::Move;
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

    fn neg(self) -> Self::Output {
        let mut new = self;
        new.mv = self.mv;
        new.eval = self.eval.wrapping_neg();
        new
    }
}

#[cfg(test)]
mod test {
    use crate::common::eval_move::EvaledMove;
    use std::cmp::{max, min};

    #[test]
    fn equal_for_same_eval() {
        let mv1 = EvaledMove::null(1);
        let mv2 = EvaledMove::null(1);
        assert_eq!(mv1, mv2);
    }

    #[test]
    fn not_equal_for_different_eval() {
        let mv1 = EvaledMove::null(1);
        let mv2 = EvaledMove::null(2);
        assert_ne!(mv1, mv2);
    }

    #[test]
    fn cmp_for_different_eval() {
        let mv1 = EvaledMove::null(1);
        let mv2 = EvaledMove::null(2);
        assert!(mv1 < mv2);
    }

    #[test]
    fn cmp_for_different_eval_neg() {
        let mv1 = EvaledMove::null(1);
        let mv2 = EvaledMove::null(-2);
        assert!(mv1 > mv2);
    }

    #[test]
    fn min_max_work() {
        let mv1 = EvaledMove::null(1);
        let mv2 = EvaledMove::null(-2);

        let max = max(mv1, mv2);
        assert_eq!(max.eval, 1);

        let min = min(mv1, mv2);
        assert_eq!(min.eval, -2);
    }
}
