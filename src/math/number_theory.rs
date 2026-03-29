pub fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
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
