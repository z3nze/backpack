use std::ops::Range;

pub struct SqrtDecomposition {
    bin_size: usize,
    _bin_count: usize,
    values: Vec<i64>,
    inc: Vec<i64>,
    sum: Vec<i64>,
}

impl SqrtDecomposition {
    pub fn new(n: usize) -> Self {
        let nsqrt = (n as f64).sqrt().max(1.0) as usize;
        let bin_size = nsqrt;
        let bin_count = (n + bin_size - 1).div_ceil(bin_size);

        let sum: Vec<i64> = vec![0; bin_count];
        let inc: Vec<i64> = vec![0; bin_count];
        let values: Vec<i64> = vec![0; n];

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
        if dv == 0 {
            return
        }

        let brange = self.block_range(block_idx);
        let block = &mut self.values[brange];

        for x in block.iter_mut() {
            *x += dv;
        }
        self.sum[block_idx] += dv * (self.bin_size as i64);
        self.inc[block_idx] = 0;
    }

    pub fn add(&mut self, l: usize, r: usize, val: i64) {
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

    pub fn sum(&mut self, l: usize, r: usize) -> i64 {
        let lbidx = self.block_idx(l);
        let rbidx = self.block_idx(r);

        self.relax_block(lbidx);
        self.relax_block(rbidx);

        let mut ret: i64 = 0;

        if lbidx == rbidx {
            for x in &self.values[l ..= r] {
                ret += x;
            }
            return ret;
        }

        for x in &self.values[l .. (lbidx + 1) * self.bin_size] {
            ret += x;
        }

        let dvs = &self.inc[lbidx + 1 .. rbidx];
        let sums = &self.sum[lbidx + 1 .. rbidx];
        for (dv, sum) in dvs.iter().zip(sums.iter()) {
            ret += dv * self.bin_size as i64 + sum;
        }

        for x in &self.values[rbidx * self.bin_size ..= r] {
            ret += x;
        }

        ret
    }

    pub fn get(&self, idx: usize) -> i64 {
        let bidx = self.block_idx(idx);
        self.values[idx] + self.inc[bidx]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_decomposition() {
        let mut sd = SqrtDecomposition::new(100);

        assert!(sd.get(54) == 0);

        sd.add(0, 49, 2);
        sd.add(39, 99, 7);

        assert!(sd.get(20) == 2);
        assert!(sd.get(45) == 9);
        assert!(sd.get(80) == 7);
        assert!(sd.sum(0, 70) == (49 - 0 + 1) * 2 + (70 - 39 + 1) * 7);

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
