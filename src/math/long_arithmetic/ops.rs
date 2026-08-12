use crate::math::long_arithmetic::schoolbook::{divide_by_multidigit_bu64, multiply_bu64};

use super::{
    schoolbook::add_signed_small_to_large_bu64,
    types::{BigInt, Sign},
};

use std::ops::{Add, Div, Mul, Neg, Sub};

impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (a, b) = if self.abs() >= rhs.abs() { (&self, &rhs) } else { (&rhs, &self) };

        let op_sign: Sign = if a.sign == b.sign { Sign::POSITIVE } else { Sign::NEGATIVE };

        let blocks = add_signed_small_to_large_bu64(&self.blocks, &rhs.blocks, op_sign);
        let sign = if blocks.is_empty() { Sign::ZERO } else { a.sign };

        BigInt { sign, blocks }
    }
}

impl Sub for BigInt {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.neg()
    }
}

fn resolve_mul_sign(lhs: Sign, rhs: Sign) -> Sign {
    match (lhs, rhs) {
        (Sign::ZERO, _) | (_, Sign::ZERO) => Sign::ZERO,
        (Sign::POSITIVE, Sign::POSITIVE) | (Sign::NEGATIVE, Sign::NEGATIVE) => Sign::POSITIVE,
        (Sign::POSITIVE, Sign::NEGATIVE) | (Sign::NEGATIVE, Sign::POSITIVE) => Sign::NEGATIVE,
    }
}

impl Mul for BigInt {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        BigInt {
            sign: resolve_mul_sign(self.sign, rhs.sign),
            blocks: multiply_bu64(&self.blocks, &rhs.blocks),
        }
    }
}

impl Div for BigInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs.sign != Sign::ZERO, "division by zero");

        let (div, _rem) = divide_by_multidigit_bu64(&self.blocks, &rhs.blocks);

        BigInt {
            sign: resolve_mul_sign(self.sign, rhs.sign),
            blocks: div,
        }
    }
}
