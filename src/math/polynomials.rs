#![allow(non_snake_case)]
use crate::math::numerical::Numerical;

pub struct LagrangeInterpolation<T>
where
    T: Numerical,
{
    x: Vec<T>,
    y: Vec<T>,
}

impl<T> LagrangeInterpolation<T>
where
    T: Numerical,
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
            .map(|i| {
                (0..i)
                    .chain(i + 1..n)
                    .map(|j| (x - self.x[j]) / (self.x[i] - self.x[j]))
                    .fold(T::lf_usize(1), |acc, v| acc * v)
                    * self.y[i]
            })
            .fold(T::lf_usize(0), |acc, v| acc + v)
    }
}

pub struct LagrangeInterpolationBarycentric<T>
where
    T: Numerical,
{
    x: Vec<T>,
    w: Vec<T>,
    yw: Vec<T>,
}

impl<T> LagrangeInterpolationBarycentric<T>
where
    T: Numerical,
{
    pub fn new(v: &[(T, T)]) -> Self {
        let x: Vec<T> = v.iter().map(|&(x, _)| x).collect::<Vec<_>>();
        let y: Vec<T> = v.iter().map(|&(_, y)| y).collect::<Vec<_>>();
        let n = x.len();
        let w: Vec<T> = (0..n)
            .map(|i| {
                T::lf_usize(1)
                    / (0..i)
                        .chain(i + 1..n)
                        .map(|j| x[i] - x[j])
                        .fold(T::lf_usize(1), |acc, v| acc * v)
            })
            .collect::<Vec<_>>();
        let yw: Vec<T> = y
            .iter()
            .zip(w.iter())
            .map(|(&a, &b)| a * b)
            .collect::<Vec<_>>();

        LagrangeInterpolationBarycentric { x, w, yw }
    }

    pub fn P(&self, x: T) -> T {
        let n = self.x.len();
        let nd = (0..n)
            .map(|i| {
                let den: T = x - self.x[i];
                (self.yw[i] / den, self.w[i] / den)
            })
            .fold((T::lf_usize(0), T::lf_usize(0)), |(num, den), (a, b)| {
                (num + a, den + b)
            });
        nd.0 / nd.1
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

    #[test]
    pub fn test_lagrange_interpolation_barycentric() {
        let points: Vec<(f64, f64)> = vec![(1.0, 1.0), (2.0, 8.0), (3.0, 27.0)];
        let li = LagrangeInterpolationBarycentric::new(&points);

        let eps: f64 = 1e-9;
        assert!((li.P(4.0) - 58.0).abs() < eps);
    }
}
