pub struct Sieve {
    is_prime: Vec<bool>,
}

impl Sieve {
    pub fn new(maxn: usize) -> Self {
        let rn: usize = (maxn as f64).sqrt() as usize;
        let mut is_prime = vec![true; maxn + 1];
        is_prime[..2].fill(false);

        for i in 2..=rn {
            if !is_prime[i] {
                continue
            }
            for j in is_prime.iter_mut().skip(i * i).step_by(i) {
                *j = false;
            }
        }
        Sieve { 
            is_prime
        }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        self.is_prime[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_sieve() {
        let maxn = 1e5 as usize;
        let ts = Sieve::new(maxn);
        let max_prime = (0..maxn + 1).rfind(|&x| ts.is_prime(x)).unwrap();
        let prime_count = (0..maxn + 1).filter(|&x| ts.is_prime(x)).count();

        assert!(!ts.is_prime(0));
        assert!(!ts.is_prime(1));
        assert!(ts.is_prime(2));
        assert!(ts.is_prime(3));
        assert!(max_prime == 99991);
        assert!(prime_count == 9592);
    }
}
