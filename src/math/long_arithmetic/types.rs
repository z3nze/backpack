use std::{cmp::Ordering, ops::Neg};

use super::schoolbook::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::math::long_arithmetic) enum Sign {
    NEGATIVE = -1,
    ZERO = 0,
    POSITIVE = 1,
}

impl Sign {
    pub fn opposite(self) -> Self {
        Sign::try_from(0 - self as i8).unwrap()
    }
}

impl TryFrom<i8> for Sign {
    type Error = &'static str;

    fn try_from(sgn: i8) -> Result<Self, Self::Error> {
        match sgn {
            -1 => Ok(Self::NEGATIVE),
            0 => Ok(Self::ZERO),
            1 => Ok(Self::POSITIVE),
            _ => Err("Sign must be from {-1, 0, 1}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BigInt {
    pub(super) sign: Sign,
    pub(super) blocks: Vec<u64>,
}

impl BigInt {
    pub fn from_u64(n: u64) -> Self {
        BigInt {
            sign: Sign::POSITIVE,
            blocks: vec![n],
        }
    }

    pub fn from_i64(n: i64) -> Self {
        BigInt {
            sign: Sign::try_from(n.signum() as i8).unwrap(),
            blocks: vec![n.abs() as u64],
        }
    }

    pub fn abs(&self) -> Self {
        BigInt {
            sign: Sign::POSITIVE,
            blocks: self.blocks.clone(),
        }
    }
}

impl Neg for BigInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        BigInt {
            sign: self.sign.opposite(),
            blocks: self.blocks.clone(),
        }
    }
}

impl PartialEq for BigInt {
    fn eq(&self, rhs: &Self) -> bool {
        let (a, b) = (&self.blocks, &rhs.blocks);
        let (n, m) = (a.len(), b.len());

        n == m && self.sign == rhs.sign && a.iter().zip(b.iter()).all(|(x, y)| x == y)
    }
}

impl Eq for BigInt {}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for BigInt {
    fn cmp(&self, rhs: &Self) -> Ordering {
        if self.sign.cmp(&rhs.sign) != Ordering::Equal {
            return self.sign.cmp(&rhs.sign);
        }

        if self.sign == Sign::POSITIVE {
            cmp(&self.blocks, &rhs.blocks)
        } else {
            cmp(&self.blocks, &rhs.blocks).reverse()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let a = BigInt {
            sign: Sign::POSITIVE,
            blocks: vec![1, 1],
        };
        let a_prime = BigInt {
            sign: Sign::POSITIVE,
            blocks: vec![1, 1],
        };
        let b = BigInt {
            sign: Sign::POSITIVE,
            blocks: vec![1, 0, 2],
        };

        assert!(a == a);
        assert_eq!(a == a_prime, a_prime == a);
        assert_eq!(a == b, b == a);
        assert!(a != b);
        assert!(b != a);
    }

    #[test]
    fn test_ord() {
        let a = BigInt {
            sign: Sign::POSITIVE,
            blocks: vec![1, 1],
        };
        let b = BigInt {
            sign: Sign::POSITIVE,
            blocks: vec![1, 0, 2],
        };
        let c = BigInt {
            sign: Sign::POSITIVE,
            blocks: vec![1, 1, 0],
        };
        let d = BigInt {
            sign: Sign::NEGATIVE,
            blocks: vec![1, 1],
        };

        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert_eq!(a.cmp(&d), Ordering::Greater);
        assert_eq!(d.cmp(&c), Ordering::Less);
        assert!(a.cmp(&b) == b.cmp(&c) && a.cmp(&b) == a.cmp(&c) && a.cmp(&c) == b.cmp(&c));
    }
}
