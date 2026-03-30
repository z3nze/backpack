use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign}};

pub trait Numerical:
    Default + Clone + Copy + Debug
    + PartialEq + PartialOrd
    + Add<Output = Self> + AddAssign
    + Sub<Output = Self> + SubAssign
    + Mul<Output = Self> + MulAssign
    + Div<Output = Self> + DivAssign
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
