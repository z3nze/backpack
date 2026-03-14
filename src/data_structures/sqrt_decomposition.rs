use std::ops::Range;

pub struct SqrtDecomposition {
    bin_size: usize,
    _bin_count: usize,
    values: Vec<i64>,
    bins: Vec<i64>,
}

impl SqrtDecomposition {
    pub fn new(n: usize) -> Self {
        let nsqrt = (n as f64).sqrt().max(1.0) as usize;
        let bin_size = nsqrt;
        let bin_count = (n + bin_size - 1).div_ceil(bin_size);

        let bins: Vec<i64> = vec![0; bin_count];
        let values: Vec<i64> = vec![0; n];

        SqrtDecomposition { bin_size, _bin_count: bin_count, values, bins }
    }

    pub fn block_idx(&self, idx: usize) -> usize {
        idx / self.bin_size
    }

    pub fn block_range(&self, block_idx: usize) -> Range<usize> {
        block_idx * self.bin_size .. (block_idx + 1) * self.bin_size
    }

    pub fn relax_block(&mut self, block_idx: usize) {
        let brange = self.block_range(block_idx);
        let block = &mut self.values[brange];
        let bval = self.bins[block_idx];

        for x in block.iter_mut() {
            *x += bval;
        }
        self.bins[block_idx] = 0;
    }

    pub fn add(&mut self, l: usize, r: usize, val: i64) {
        let lbidx = self.block_idx(l);
        let rbidx = self.block_idx(r);

        self.relax_block(lbidx);
        self.relax_block(rbidx);

        for x in &mut self.values[l..(lbidx + 1) * self.bin_size] {
            *x += val;
        }
        for x in &mut self.bins[lbidx + 1 .. rbidx] {
            *x += val;
        }
        for x in &mut self.values[rbidx * self.bin_size ..= r] {
            *x += val;
        }
    }

    pub fn get(&self, idx: usize) -> i64 {
        let bidx = self.block_idx(idx);
        self.values[idx] + self.bins[bidx]
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

        sd.add(0, 99, -100);

        assert!(sd.get(50) == -93);
        assert!(sd.get(39) == -91);
        assert!(sd.get(0) == -98);
    }
}
