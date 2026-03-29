use crate::math::continued_fractions::QuadraticIrrational;

pub enum Etype {
    Positive,
    Negative,
}

pub struct PellsEquation {
    et: Etype,
    _n: usize,
}

pub struct Solution {
}

impl PellsEquation {
    pub fn new(n: usize, et: Etype) -> Self {
        let rn = n.isqrt();
        if n == 0 || rn * rn == n {
            panic!("n must be positive non-square integer")
        }
        PellsEquation { et, _n: n }
    }

    fn solve_positive(&self) {
    }

    fn solve_negative(&self) {
        let qi = QuadraticIrrational::new(self._n as i64);
    }

    pub fn solve(&self) {
        match self.et {
            Etype::Positive => self.solve_positive(),
            Etype::Negative => self.solve_negative(),
        }
    }
}
