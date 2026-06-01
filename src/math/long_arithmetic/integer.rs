use std::{cmp::Ordering, iter::repeat, ops::{Add, Div, Mul}};

#[derive(Debug, Clone)]
pub struct Vanilla {
    blocks: Vec<u64>
}

impl Vanilla {
    pub fn new(n: u64) -> Self {
        Vanilla {
            blocks: vec![n; 1]
        }
    }
}

impl Mul for Vanilla {
    type Output = Self;

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

        Vanilla { blocks }
    }
}

impl Add for Vanilla {
    type Output = Self;

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

        Vanilla { blocks }
    }
}

impl Div for Vanilla {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl PartialEq for Vanilla {
    fn eq(&self, rhs: &Self) -> bool {
        let a = &self.blocks;
        let b = &rhs.blocks;
        let n = a.len();
        let m = b.len();

        n == m && a.iter().zip(b.iter()).all(|(a_i, b_i)| a_i == b_i)
    }
}

impl Eq for Vanilla {}

impl PartialOrd for Vanilla {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Vanilla {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let a = &self.blocks;
        let b = &rhs.blocks;
        let n = a.len();
        let m = b.len();

        if n != m {
            return n.cmp(&m);
        }

        if let Some(neq) = a.iter().zip(b.iter()).find(|(a_i, b_i)| a_i != b_i) {
            return neq.0.cmp(neq.1);
        }

        Ordering::Equal
    }
}
