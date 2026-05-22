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
    pub fn new(v: &[(T, T)]) -> Self {
        LagrangeInterpolation {
            x: v.iter().map(|&(x, _)| x).collect::<Vec<_>>(),
            y: v.iter().map(|&(_, y)| y).collect::<Vec<_>>(),
        }
    }

    pub fn P(&self, x: T) -> T {
        let n = self.x.len();
        (0..n)
            .map(|i| 
                (0..i).chain(i + 1 .. n)
                .map(|j| (x - self.x[j]) / (self.x[i] - self.x[j]))
                .fold(T::lf_usize(1), |acc, v| acc * v) * self.y[i]
            )
            .fold(T::lf_usize(0), |acc, v| acc + v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_lagrange_interpolation() {
        let points: Vec<(f64, f64)> = vec![(1.0, 1.0), (2.0, 8.0)];
        let li = LagrangeInterpolation::new(&points);

        assert_eq!(li.P(3.0), 15.0);
    }
}
