use std::cell::Cell;

pub struct Sieve {
    is_prime: Vec<Cell<bool>>,
}

impl Sieve {
    pub fn new(maxn: usize) -> Self {
        let rn: usize = maxn.isqrt(); 
        let mut is_prime = vec![Cell::new(true); maxn + 1];
        is_prime[..2].fill(Cell::new(false));

        is_prime.iter().enumerate().take(rn + 1).skip(2)
            .filter(|(_, prime)| prime.get())
            .for_each(|(i, _)| is_prime.iter().skip(i * i).step_by(i).for_each(|j| j.set(false)));

        Sieve { 
            is_prime
        }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        self.is_prime[n].get()
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
