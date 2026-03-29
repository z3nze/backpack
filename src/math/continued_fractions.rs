use crate::math::number_theory::gcd;

pub struct QuadraticIrrational {
    _d: i64,
    a: Vec<i64>,
}


// Simple continued fraction algo.
impl QuadraticIrrational {
    pub fn new(d: i64) -> Self {
        let mut m_k: i64 = 0;
        let mut d_k: i64 = 1;
        let mut a: Vec<i64> = vec![];

        let mut md_1: Option<(i64, i64)> = None;

        while md_1.is_none() || Some((m_k, d_k)) != md_1 {
            let a_k = ((m_k as f64 + (d as f64).sqrt()) / (d_k as f64)).floor() as i64;
            let m_nk: i64 = a_k * d_k - m_k;
            let d_nk: i64 = (d - m_nk.pow(2)) / d_k;

            a.push(a_k);

            if a.len() == 2 {
                md_1 = Some((m_k, d_k));
            }

            m_k = m_nk;
            d_k = d_nk;
        }

        QuadraticIrrational{ _d: d, a }
    }

    pub fn convergent(&self, k: usize) -> (i64, i64) {
        let mut num: i64 = 0;
        let mut den: i64 = 1;

        for a_i in self.a.iter().take(k).rev() {
            num += den * a_i;
            (num, den) = (den, num);
            let g = gcd(num, den);
            num /= g;
            den /= g;
        }
        (den, num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_quadratic_irrational() {
        let qi2 = QuadraticIrrational::new(2);
        let expect_cf = vec![1, 2];

        assert_eq!(qi2.a, expect_cf);

        let qi19 = QuadraticIrrational::new(19);
        let expect_cf_19 = vec![4, 2, 1, 3, 1, 2, 8];

        assert_eq!(qi19.a, expect_cf_19);
    }

    #[test]
    pub fn test_convergents() {
        let qi19 = QuadraticIrrational::new(19);

        assert_eq!(qi19.convergent(4), (48, 11));
    }
}
