use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait FromUsize {
    fn from_usize(n: usize) -> Self;
}

impl FromUsize for i64 {
    fn from_usize(n: usize) -> i64 {
        n as i64
    }
}

pub trait Numerical:
    Default + Clone + Copy
    + Eq
    + Add<Output = Self> + AddAssign
    + Sub<Output = Self> + SubAssign
    + Mul<Output = Self> + MulAssign
    + Div<Output = Self> + DivAssign
    + FromUsize
{}

impl<U> Numerical for U
where
    U:
        Default + Clone + Copy
        + Eq
        + Add<Output = Self> + AddAssign
        + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign
        + Div<Output = Self> + DivAssign
        + FromUsize,
{}
