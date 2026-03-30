use crate::math::continued_fractions::QuadraticIrrational;

pub enum Etype {
    Positive,
    Negative,
}

pub struct PellsEquation {
    et: Etype,
    _n: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Solution {
    x: i64,
    y: i64,
}

impl PellsEquation {
    pub fn new(n: usize, et: Etype) -> Self {
        let rn = n.isqrt();
        if n == 0 || rn * rn == n {
            panic!("n must be positive non-square integer")
        }
        PellsEquation { et, _n: n }
    }

    fn solve_positive(&self) -> Option<Solution> {
        None
    }

    fn solve_negative(&self) -> Option<Solution> {
        let qi = QuadraticIrrational::new(self._n as i64);
        let pl = qi.period_length();
        if pl.is_multiple_of(2) {
            return None;
        }
        let (x, y) = qi.convergent(pl);
        Some(Solution{ x, y })
    }

    pub fn solve(&self) -> Option<Solution> {
        match self.et {
            Etype::Positive => self.solve_positive(),
            Etype::Negative => self.solve_negative(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_pells_equation_negative_solution_exists() {
        let d: i64 = 85;
        let pe = PellsEquation::new(d as usize, Etype::Negative);
        let sol = pe.solve().unwrap();
        assert_eq!(sol.x.pow(2) - d * sol.y.pow(2), -1);
    }

    #[test]
    pub fn test_pells_equation_no_solution() {
        let d: i64 = 7;
        let pe = PellsEquation::new(d as usize, Etype::Negative);
        let sol = pe.solve();
        assert!(sol.is_none());
    }
}
