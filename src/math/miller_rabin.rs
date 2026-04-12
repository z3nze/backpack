#![allow(clippy::just_underscores_and_digits)]

use std::time::SystemTime;
use crate::{math::numerical::{Integer}, misc::random::Xoshiro256ppGenerator};

pub fn powmod<T: Integer>(base: T, e: T, m: T) -> T {
    let _0 = T::try_from(0).unwrap();
    let _1 = T::try_from(1).unwrap();
    let _2 = T::try_from(2).unwrap();

    if e == _0 {
        return _1;
    }
if e % _2 == _0 {
        let eh = powmod(base, e / _2, m);
        return (eh * eh) % m;
    }
    (base * powmod(base, e - _1, m)) % m
}


pub fn check_composite<T: Integer>(n: T, a: T, d: T, s: usize) -> bool {
    let _1 = T::try_from(1).unwrap();
    let mut x: T = powmod(a, d, n);

    if x == _1 || x == n - _1 {
        return false;
    }

    for _ in 1..s {
        x = (x * x) % n;
        if x == n - _1 {
            return false;
        }
    }

    true
}

pub fn is_prime<T: Integer>(n: T, iter: usize) -> bool {
    let _4 = T::try_from(4).unwrap();
    let _2 = T::try_from(2).unwrap();
    let _3 = T::try_from(3).unwrap();
    let _1 = T::try_from(1).unwrap();
    let _0 = T::try_from(0).unwrap();

    if n < _4 {
        return n == _2 || n == _3;
    }

    let mut s = 0;
    let mut d: T = n - _1;
    while d & _1 == _0 {
        d >>= _1;
        s += 1;
    }

    let now = SystemTime::now();
    let mut rg = Xoshiro256ppGenerator::new(now.elapsed().unwrap().as_secs());

    for _ in 0..iter {
        let a: T = _2 + T::try_from(rg.rand() as usize).unwrap() % (n - _3);
        if check_composite(n, a, d, s) {
            return false;
        }
    }
    true 
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn test_is_prime() {
        assert!(!is_prime(4u64, 10));
        assert!(is_prime(1000000007u64, 10));
        assert!(!is_prime(12934692u64, 10));
    }
}
