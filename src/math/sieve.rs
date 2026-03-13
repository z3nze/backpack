use std::num::NonZeroUsize;

pub struct Sieve {
    maxn: usize,
    is_prime: Vec<bool>,
}

impl Sieve {
    pub fn new(maxn: NonZeroUsize) -> Self {
        let n: usize = maxn.get();
        let rn: usize = (n as f64).sqrt() as usize;
        let mut is_prime = vec![true; n + 1];
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
            maxn: n, 
            is_prime
        }
    }

    pub fn is_prime(&self, n: usize) -> Option<bool> {
        match n {
            x if x > self.maxn => None,
            _ => Some(self.is_prime[n])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_sieve() {
        let maxn = 1e5 as usize;
        let ts = Sieve::new(NonZeroUsize::new(maxn).unwrap());
        let max_prime = (0..maxn + 1).rfind(|&x| ts.is_prime(x).unwrap()).unwrap();
        let prime_count = (0..maxn + 1).filter(|&x| ts.is_prime(x).unwrap()).count();

        assert!(!ts.is_prime(0).unwrap());
        assert!(!ts.is_prime(1).unwrap());
        assert!(ts.is_prime(2).unwrap());
        assert!(ts.is_prime(3).unwrap());
        assert!(ts.is_prime(maxn + 1).is_none());
        assert!(max_prime == 99991);
        assert!(prime_count == 9592);
    }
}
