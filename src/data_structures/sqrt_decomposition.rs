use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Range, Sub, SubAssign};

pub struct SqrtDecomposition<T> {
    bin_size: usize,
    _bin_count: usize,
    values: Vec<T>,
    inc: Vec<T>,
    sum: Vec<T>,
}

pub trait FromUsize {
    fn from_usize(n: usize) -> Self;
}

impl FromUsize for i64 {
    fn from_usize(n: usize) -> i64 {
        n as i64
    }
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
    + SubAssign
    + Mul<Output = T>
    + MulAssign
    + Div<Output = T>
    + DivAssign
    + FromUsize
{
    pub fn new(n: usize) -> Self
    {
        let nsqrt = (n as f64).sqrt().max(1.0) as usize;
        let bin_size = nsqrt;
        let bin_count = (n + bin_size - 1).div_ceil(bin_size);

        let sum: Vec<T> = vec![T::default(); bin_count];
        let inc: Vec<T> = vec![T::default(); bin_count];
        let values: Vec<T> = vec![T::default(); n];

        SqrtDecomposition { bin_size, _bin_count: bin_count, values, inc, sum }
    }

    pub fn block_idx(&self, idx: usize) -> usize {
        idx / self.bin_size
    }

    pub fn block_range(&self, block_idx: usize) -> Range<usize> {
        block_idx * self.bin_size .. (block_idx + 1) * self.bin_size
    }

    pub fn relax_block(&mut self, block_idx: usize) {
        let dv = self.inc[block_idx];
        if dv == T::default() {
            return
        }

        let brange = self.block_range(block_idx);
        let block = &mut self.values[brange];

        for x in block.iter_mut() {
            *x += dv;
        }
        self.sum[block_idx] += T::from_usize(self.bin_size) * dv;
        self.inc[block_idx] = T::default();
    }

    pub fn add(&mut self, l: usize, r: usize, val: T)
    where 
        T: Default
    {
        let lbidx = self.block_idx(l);
        let rbidx = self.block_idx(r);

        self.relax_block(lbidx);
        self.relax_block(rbidx);

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

        for x in &mut self.inc[lbidx + 1 .. rbidx] {
            *x += val;
        }

        for x in &mut self.values[rbidx * self.bin_size ..= r] {
            *x += val;
            self.sum[rbidx] += val;
        }
    }

    pub fn sum(&mut self, l: usize, r: usize) -> T {
        let lbidx = self.block_idx(l);
        let rbidx = self.block_idx(r);

        self.relax_block(lbidx);
        self.relax_block(rbidx);

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

        let dvs = &self.inc[lbidx + 1 .. rbidx];
        let sums = &self.sum[lbidx + 1 .. rbidx];
        for (&dv, &sum) in dvs.iter().zip(sums.iter()) {
            ret += T::from_usize(self.bin_size) * dv + sum;
        }

        for &x in &self.values[rbidx * self.bin_size ..= r] {
            ret += x;
        }

        ret
    }

    pub fn get(&self, idx: usize) -> T {
        let bidx = self.block_idx(idx);
        self.values[idx] + self.inc[bidx]
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
