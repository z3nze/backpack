use std::{iter::repeat, ops::{Add, Div, Mul}};

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
    fn mul(self, rhs: Self) -> Self::Output {
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

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Self) -> Self::Output {
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

impl Div for NaiveLongInt {
    type Output = NaiveLongInt;

    fn div(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
