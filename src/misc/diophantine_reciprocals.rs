use crate::math::sieve::Sieve;

/// Finds first $n \leq MAXN$ such that number of solutions $\frac{1}{x} + \frac{1}{y} = \frac{1}{n}$ exceedes 1000.
pub fn first_exceeding_1000(maxn: usize) -> Option<usize> {
    let sieve = Sieve::new(maxn);

    let mut a: Vec<usize> = vec![1; maxn + 1];
    for p in 2..=maxn {
        if !sieve.is_prime(p) {
            continue;
        }
        for j in (p..=maxn).step_by(p) {
            let mut tj = j;
            let mut a_p = 0;
            while tj % p == 0 {
                tj /= p;
                a_p += 1;
            }
            a[j] *= 2 * a_p + 1;
        }
    }
    for n in 2..=maxn {
        a[n] = (a[n] + 1) / 2;
        if a[n] > 1000 {
            return Some(n);
        }
    }
    None
}
