use std::{cmp::Ordering, iter::repeat, ops::{Add, Sub, Div, Mul, Neg}};

#[derive(Debug, Clone)]
pub struct Vanilla {
    neg: bool,
    blocks: Vec<u64>
}

impl Vanilla {
    pub fn from_u64(n: u64) -> Self {
        Vanilla {
            neg: false,
            blocks: vec![n],
        }
    }

    pub fn from_i64(n: i64) -> Self {
        Vanilla {
            neg: n < 0,
            blocks: vec![n.abs() as u64],
        }
    }

    pub fn abs(&self) -> Self {
        Vanilla {
            neg: false,
            blocks: self.blocks.clone(),
        }
    }

    fn add_small_to_large(&self, rhs: Self) -> Self {
        let base: i128 = 1i128 << 64;
        let mut a = self;
        let mut b = &rhs;

        let mut n = a.blocks.len();
        let mut m = b.blocks.len();
        let max_len = n.max(m) + 1;

        if a.abs() < b.abs() {
            (a, b) = (b, a);
            (n, m) = (m, n);
        }

        let b_sign: i128 = if a.neg != b.neg { -1 } else { 1 };
        let neg: bool = a.neg;


        let mut blocks: Vec<u64> = vec![0; max_len];
        let mut carry: i64 = 0;

        a.blocks.iter().chain(repeat(&0)).take(max_len)
            .zip(b.blocks.iter().chain(repeat(&0)).take(max_len))
            .zip(blocks.iter_mut())
            .for_each(|((&ai, &bi), ref mut block_i)| {
                let res: i128 = (**block_i as i128) + (ai as i128) + b_sign * (bi as i128) + (carry as i128);
                carry = (res.div_euclid(base)) as i64;
                **block_i = res.rem_euclid(base) as u64;
            });

        if blocks.len() > 1 && blocks.last().unwrap() == &0 {
            blocks.pop();
        }

        Vanilla {
            neg: neg,
            blocks: blocks,
        }
    }

    fn cmp_abs(&self, rhs: &Self) -> Ordering {
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

impl Neg for Vanilla {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vanilla {
            neg: !self.neg,
            blocks: self.blocks.clone(),
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

        Vanilla {
            neg: self.neg != rhs.neg,
            blocks: blocks
        }
    }
}

impl Add for Vanilla {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Self) -> Self::Output {
        self.add_small_to_large(rhs)
    }
}

impl Sub for Vanilla {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, rhs: Self) -> Self::Output {
        self.add_small_to_large(rhs.neg())
    }
}

impl Div for Vanilla {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self::Output {
        let base: u128 = 1u128 << 64;
        let d_last = *_rhs.blocks.last().unwrap();
        let f = base.div_ceil(d_last as u128) as u64;

        let a_prime = self * Self::from_u64(f);
        let d_prime = _rhs * Self::from_u64(f);

        let mut rem = a_prime;
        unimplemented!()
    }
}

impl PartialEq for Vanilla {
    fn eq(&self, rhs: &Self) -> bool {
        let a = &self.blocks;
        let b = &rhs.blocks;
        let n = a.len();
        let m = b.len();

        n == m && self.neg == rhs.neg && a.iter().zip(b.iter()).all(|(a_i, b_i)| a_i == b_i)
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
        match (self.neg, rhs.neg) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (true, true) => self.cmp_abs(rhs).reverse(),
            (false, false) => self.cmp_abs(rhs),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let a = Vanilla {
            neg: false,
            blocks: vec![1, 1],
        };
        let a_prime = Vanilla {
            neg: false,
            blocks: vec![1, 1],
        };
        let b = Vanilla {
            neg: false,
            blocks: vec![1, 0, 2]
        };

        assert!(a == a);
        assert_eq!(a == a_prime, a_prime == a);
        assert_eq!(a == b, b == a);
        assert!(a != b);
        assert!(b != a);
    }

    #[test]
    fn test_ord() {
        let a = Vanilla {
            neg: false,
            blocks: vec![1, 1]
        };
        let b = Vanilla {
            neg: false,
            blocks: vec![1, 0, 2]
        };
        let c = Vanilla {
            neg: false,
            blocks: vec![1, 1, 0]
        };

        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert!(a.cmp(&b) == b.cmp(&c) && a.cmp(&b) == a.cmp(&c) && a.cmp(&c) == b.cmp(&c));
    }
}
