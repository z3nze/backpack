use std::{fmt::Debug, ops::{Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign}};

pub trait Numerical:
    Default + Clone + Copy + Debug
    + PartialEq + PartialOrd
    + Add<Output = Self> + AddAssign
    + Sub<Output = Self> + SubAssign
    + Mul<Output = Self> + MulAssign
    + Div<Output = Self> + DivAssign
    + Shr<Output = Self> + ShrAssign
    + Shl<Output = Self> + ShlAssign
    + BitAnd<Output = Self> + BitAndAssign
    + TryFrom<usize, Error: Debug>
{}

impl<U> Numerical for U
where
    U:
        Default + Clone + Copy + Debug
        + PartialEq + PartialOrd
        + Add<Output = Self> + AddAssign
        + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign
        + Div<Output = Self> + DivAssign
        + Shr<Output = Self> + ShrAssign
        + Shl<Output = Self> + ShlAssign
        + BitAnd<Output = Self> + BitAndAssign
        + TryFrom<usize, Error: Debug>
{}

pub trait Integer:
    Numerical
    + Eq + Ord
    + Rem<Output = Self> + RemAssign
{}

impl<U> Integer for U
where 
    U:
        Numerical
        + Eq + Ord
        + Rem<Output = Self> + RemAssign
{}
