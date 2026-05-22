#![allow(non_snake_case)]
use crate::math::numerical::Numerical;

pub struct LagrangeInterpolation<T>
where
    T: Numerical
{
    x: Vec<T>,
    y: Vec<T>,
}

impl<T> LagrangeInterpolation<T>
where 
    T: Numerical
{
    pub fn new(v: Vec<(T, T)>) -> Self {
        LagrangeInterpolation {
            x: v.iter().map(|&(x, _)| x).collect::<Vec<_>>(),
            y: v.iter().map(|&(_, y)| y).collect::<Vec<_>>(),
        }
    }

    pub fn P(&self, x: T) -> T {
        let n = self.x.len();
        let mut P: T = T::lf_usize(0);
        for i in 0..n {
            let mut l: T = T::lf_usize(1);
            for j in 0..n {
                if i == j {
                    continue
                }
                l = l * (x - self.x[j]) / (self.x[i] - self.x[j]);
            }
            P += self.y[i] * l;
        }
       P 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_lagrange_interpolation() {
        let points: Vec<(f64, f64)> = vec![(1.0, 1.0), (2.0, 8.0)];
        let li = LagrangeInterpolation::new(points);

        assert_eq!(li.P(3.0), 15.0);
    }
}
