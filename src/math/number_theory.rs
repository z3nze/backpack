#![allow(clippy::just_underscores_and_digits)]

use crate::math::numerical::Integer;

pub fn gcd<T: Integer>(a: T, b: T) -> T {
    if a == T::try_from(0).unwrap() {
        return b;
    }
    gcd(b % a, a)
}

pub fn phi<T: Integer>(mut n: T) -> T {
    let _0 = T::try_from(0).unwrap();
    let _1 = T::try_from(1).unwrap();
    let _2 = T::try_from(2).unwrap();

    let mut result = n;
    let mut i = _2;
    while i * i <= n {
        if n % i == _0 {
            while n % i == _0 {
                n /= i;
            }
            result -= result / i;
        }
        i += _1;
    }

    if n > _1 {
        result -= result / n;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_gcd() {
        assert_eq!(gcd(24, 17), 1);
        assert_eq!(gcd(259457, 2541), 11);
    }

    #[test]
    pub fn test_phi() {
        assert_eq!(phi(11), 10);
        assert_eq!(phi(18), 6);
    }
}
