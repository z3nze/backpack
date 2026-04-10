use crate::math::numerical::Integer;

pub fn powmod<T: Integer>(base: T, e: T, m: T) -> T {
    if e == T::try_from(0).unwrap() {
        return T::try_from(1).unwrap();
    }
    if e % T::try_from(2).unwrap() == T::try_from(0).unwrap() {
        let eh = powmod(base, e / T::try_from(2).unwrap(), m);
        return (eh * eh) % m;
    }
    (base * powmod(base, e - T::try_from(1).unwrap(), m)) % m
}

pub fn check_composite<T: Integer>(n: T, a: T, d: T, s: usize) -> bool {
    unimplemented!()
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
