pub fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}
