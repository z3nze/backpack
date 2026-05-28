use std::{iter::repeat, fmt::Debug, ops::{Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign}};

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

pub struct NaiveLongInt {
    blocks: Vec<u64>
}

impl NaiveLongInt {
    pub fn new(n: u64) -> Self {
        NaiveLongInt {
            blocks: vec![n; 1]
        }
    }
}

impl Mul for NaiveLongInt {
    type Output = NaiveLongInt;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: NaiveLongInt) -> Self {
        let a = &self.blocks;
        let b = &rhs.blocks;
        let n = a.len();
        let m = b.len();
        
        let mut blocks: Vec<u64> = vec![0; n * m + 1];

        a.iter().enumerate().for_each(|(i, &ai)| {
            b.iter().enumerate().for_each(|(j, &bj)| {
                    let uv = blocks[i * j] as u128 + (ai as u128) * (bj as u128);
                    let carry = uv >> 64;
                    let val = uv as u64;
                    blocks[i * j + 1] += carry as u64;
                    blocks[i * j] = val;
                })
            });

        if let Some(last) = blocks.last() && *last == 0 && blocks.len() > 1 {
            blocks.pop();
        }

        NaiveLongInt { blocks }
    }
}

impl Add for NaiveLongInt {
    type Output = NaiveLongInt;

    fn add(self, rhs: NaiveLongInt) -> Self {
        let a = &self.blocks;
        let b = &rhs.blocks;
        let n = a.len();
        let m = b.len();
        let max_len = n.max(m) + 1;

        let mut blocks: Vec<u64> = vec![0; max_len];
        let mut carry: u64 = 0;

        a.iter().chain(repeat(&0)).take(max_len)
            .zip(b.iter().chain(repeat(&0)).take(max_len))
            .zip(blocks.iter_mut())
            .for_each(|((&ai, &bi), ref mut block_i)| {
                let res: u128 = (**block_i as u128) + (ai as u128) * (bi as u128) + (carry as u128);
                carry = (res >> 64) as u64;
                **block_i = res as u64;
            });

        NaiveLongInt { blocks }
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
