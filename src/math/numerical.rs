use std::{fmt::Debug, ops::{Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign}};

pub trait LossyFromUsize {
    fn lf_usize(n: usize) -> Self;
}

impl LossyFromUsize for i64 {
    fn lf_usize(n: usize) -> Self {
        n as i64
    }
}

impl LossyFromUsize for u64 {
    fn lf_usize(n: usize) -> Self {
        n as u64
    }
}

impl LossyFromUsize for i32 {
    fn lf_usize(n: usize) -> Self {
        n as i32
    }
}

impl LossyFromUsize for f64 {
    fn lf_usize(n: usize) -> Self {
        n as f64
    }
}

pub trait Numerical:
    Default + Clone + Copy + Debug
    + PartialEq + PartialOrd
    + Add<Output = Self> + AddAssign
    + Sub<Output = Self> + SubAssign
    + Mul<Output = Self> + MulAssign
    + Div<Output = Self> + DivAssign
    + LossyFromUsize
{
}


impl<U> Numerical for U
where
    U:
        Default + Clone + Copy + Debug
        + PartialEq + PartialOrd
        + Add<Output = Self> + AddAssign
        + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign
        + Div<Output = Self> + DivAssign
        + LossyFromUsize
{}


pub trait Integer:
    Numerical
    + Eq + Ord
    + Shr<Output = Self> + ShrAssign
    + Shl<Output = Self> + ShlAssign
    + BitAnd<Output = Self> + BitAndAssign
    + Rem<Output = Self> + RemAssign
{}


impl<U> Integer for U
where 
    U:
        Numerical
        + Eq + Ord
        + Shr<Output = Self> + ShrAssign
        + Shl<Output = Self> + ShlAssign
        + BitAnd<Output = Self> + BitAndAssign
        + Rem<Output = Self> + RemAssign
{}
