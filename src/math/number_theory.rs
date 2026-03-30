use crate::math::numerical::Integer;

pub fn gcd<T: Integer>(a: T, b: T) -> T {
    if a == T::try_from(0).unwrap() {
        return b;
    }
    gcd(b % a, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_gcd() {
        assert_eq!(gcd(24, 17), 1);
        assert_eq!(gcd(259457, 2541), 11);
    }
}
