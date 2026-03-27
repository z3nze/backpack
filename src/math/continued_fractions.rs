use std::collections::HashMap;

pub struct QuadraticIrrational {
    _d: i64,
    _non_periodic: Vec<(i64, i64)>,
    _period: Vec<(i64, i64)>,
}

impl QuadraticIrrational {
    pub fn new(d: i64) -> Self {
        let mut non_periodic: Vec<(i64, i64)> = vec![];
        let mut period: Vec<(i64, i64)> = vec![];
        let period_length;

        let mut pos: HashMap<(i64, i64), usize> = Default::default();

        let mut a: Vec<i64> = vec![0];
        let mut b: Vec<i64> = vec![d.isqrt()];

        pos.insert((0, d.isqrt()), 0);
        let mut acc: i64 = 0;
        loop {
            let tb = d.isqrt() + acc;
            acc = tb - acc;
            let ta = d - acc.pow(2);
            let idx = pos.len();
            if let Some(pidx) = pos.get(&(ta, tb)) {
                period_length = idx - pidx;
                break;
            }
            a.push(ta);
            b.push(tb);
            pos.insert((ta, tb), idx);
        }

        for i in 1..a.len() - period_length {
            non_periodic.push((a[i], b[i]));
        }
        for i in a.len() - period_length .. a.len() {
            period.push((a[i], b[i]));
        }

        QuadraticIrrational{ _d: d, _non_periodic: non_periodic, _period: period }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_quadratic_irrational() {
        let qi = QuadraticIrrational::new(2);
        let expect_non_periodic = vec![(1, 1)];
        let expect_period = vec![(1, 2)];

        assert_eq!(qi._non_periodic, expect_non_periodic);
        assert_eq!(qi._period, expect_period);
    }
}
