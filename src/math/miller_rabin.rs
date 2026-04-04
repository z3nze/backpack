use crate::math::numerical::Integer;

pub fn check_composite<T: Integer>(n: Integer, a: Integer, d: Integer, s: usize) -> bool {
    false
}

pub fn is_prime<T: Integer>(n: T, iter: usize) -> bool {
    if n < T::try_from(4).unwrap() {
        return n == T::try_from(2).unwrap() || n == T::try_from(3).unwrap();
    }

    let mut s = 0;
    let mut d: T = n - T::try_from(1).unwrap();
    while d & T::try_from(1).unwrap() == T::try_from(0).unwrap() {
        d >>= T::try_from(1).unwrap();
        s += 1;
    }
    true 
}
