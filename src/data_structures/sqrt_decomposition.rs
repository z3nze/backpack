use std::{iter::repeat_n, ops::{Add, AddAssign, Div, Mul, Range, Sub}};

pub struct SqrtDecomposition<T> {
    bin_size: usize,
    _bin_count: usize,
    values: Vec<T>,
    sum: Vec<T>,
}

impl<T> SqrtDecomposition<T>
where 
    T: Default
        + Clone
        + Copy
        + Eq
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
{
    pub fn new(n: usize) -> Self
    {
        let nsqrt = (n as f64).sqrt().max(1.0) as usize;
        let bin_size = nsqrt;
        let bin_count = (n + bin_size - 1).div_ceil(bin_size);

        let sum: Vec<T> = vec![T::default(); bin_count];
        let values: Vec<T> = vec![T::default(); n];

        SqrtDecomposition { bin_size, _bin_count: bin_count, values, sum }
    }

    pub fn block_idx(&self, idx: usize) -> usize {
        idx / self.bin_size
    }

    pub fn block_range(&self, block_idx: usize) -> Range<usize> {
        block_idx * self.bin_size .. (block_idx + 1) * self.bin_size
    }

    pub fn add(&mut self, l: usize, r: usize, val: T)
    where 
        T: Default
    {
        let lbidx = self.block_idx(l);
        let rbidx = self.block_idx(r);

        if lbidx == rbidx {
            for x in &mut self.values[l ..= r] {
                *x += val;
                self.sum[lbidx] += val;
            }
            return
        }

        for x in &mut self.values[l .. (lbidx + 1) * self.bin_size] {
            *x += val;
            self.sum[lbidx] += val;
        }

        let dv = repeat_n(val, self.bin_size).fold(T::default(), |acc, x| acc + x);

        for x in &mut self.sum[lbidx + 1 .. rbidx] {
            *x += dv;
        }

        for x in &mut self.values[rbidx * self.bin_size ..= r] {
            *x += val;
            self.sum[rbidx] += val;
        }
    }

    pub fn sum(&mut self, l: usize, r: usize) -> T {
        let lbidx = self.block_idx(l);
        let rbidx = self.block_idx(r);

        let mut ret: T = T::default();

        if lbidx == rbidx {
            for &x in &self.values[l ..= r] {
                ret += x;
            }
            return ret;
        }

        for &x in &self.values[l .. (lbidx + 1) * self.bin_size] {
            ret += x;
        }

        ret += self.sum[lbidx + 1 .. rbidx].iter().fold(T::default(), |acc, &x| acc + x);

        for &x in &self.values[rbidx * self.bin_size ..= r] {
            ret += x;
        }

        ret
    }

    pub fn get(&self, idx: usize) -> T {
        self.values[idx]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_decomposition() {
        let mut sd: SqrtDecomposition<i64> = SqrtDecomposition::new(100);

        assert!(sd.get(54) == 0);

        sd.add(0, 49, 2);
        sd.add(39, 99, 7);

        assert!(sd.get(20) == 2);
        assert!(sd.get(45) == 9);
        assert!(sd.get(80) == 7);
        assert!(sd.sum(0, 70) == 324); // (49 - 0 + 1) * 2 + (70 - 39 + 1) * 7)

        sd.add(0, 99, -100);

        assert!(sd.get(50) == -93);
        assert!(sd.get(39) == -91);
        assert!(sd.get(0) == -98);
        assert!(sd.sum(35, 99) == (49 - 35 + 1) * 2 + (99 - 39 + 1) * 7 + (99 - 35 + 1) * -100);

        sd.add(99, 99, 100);
        assert!(sd.get(99) == 7);
        assert!(sd.sum(99, 99) == 7);
    }
}
