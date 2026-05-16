use std::{fmt::Debug, ops::{Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign}};

pub trait FromUsizeLossy {
    fn from_usize_lossy(n: usize) -> Self;
}

impl FromUsizeLossy for i64 {
    fn from_usize_lossy(n: usize) -> Self {
        n as i64
    }
}

impl FromUsizeLossy for u64 {
    fn from_usize_lossy(n: usize) -> Self {
        n as u64
    }
}

impl FromUsizeLossy for i32 {
    fn from_usize_lossy(n: usize) -> Self {
        n as i32
    }
}

impl FromUsizeLossy for f64 {
    fn from_usize_lossy(n: usize) -> Self {
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
    + FromUsizeLossy
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
        + FromUsizeLossy
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
