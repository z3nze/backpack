use std::collections::HashMap;

pub struct QuadraticIrrational {
    _d: i64,
    _non_periodic: Vec<i64>,
    _period: Vec<i64>,
}


// Simple continued fraction algo.
impl QuadraticIrrational {
    pub fn new(d: i64) -> Self {
        let mut non_periodic: Vec<i64> = vec![];
        let mut period: Vec<i64> = vec![];
        let period_length;

        let mut pos: HashMap<(i64, i64), usize> = Default::default();

        let mut m_k: i64 = 0;
        let mut d_k: i64 = 1;
        let mut a: Vec<i64> = vec![];

        pos.insert((m_k, d_k), 0);
        loop {
            let a_k = ((m_k as f64 + (d as f64).sqrt()) / (d_k as f64)).floor() as i64;
            a.push(a_k);
            let m_nk: i64 = a_k * d_k - m_k;
            let d_nk: i64 = (d - m_nk.pow(2)) / d_k;

            let idx = pos.len();
            if let Some(pidx) = pos.get(&(m_nk, d_nk)) {
                period_length = idx - pidx;
                break;
            }
            pos.insert((m_nk, d_nk), idx);
            m_k = m_nk;
            d_k = d_nk;
        }

        non_periodic = a[0..a.len() - period_length].to_vec();
        period = a[a.len() - period_length .. a.len()].to_vec();

        dbg!(a);

        QuadraticIrrational{ _d: d, _non_periodic: non_periodic, _period: period }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_quadratic_irrational() {
        let qi2 = QuadraticIrrational::new(2);
        let expect_non_periodic = vec![1];
        let expect_period = vec![2];

        assert_eq!(qi2._non_periodic, expect_non_periodic);
        assert_eq!(qi2._period, expect_period);

        let qi19 = QuadraticIrrational::new(19);
        let expect_non_periodic = vec![4];
        let expect_period = vec![2, 1, 3, 1, 2, 8];

        assert_eq!(qi19._non_periodic, expect_non_periodic);
        assert_eq!(qi19._period, expect_period);
    }
}
