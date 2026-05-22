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
}
