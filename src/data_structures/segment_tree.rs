pub struct SegmentTree {
    buf: Vec<i64>
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        let buf: Vec<i64> = vec![0; 4 * n];
        SegmentTree { buf }
    }

    pub fn add(&mut self, l: usize, r: usize, val: usize) {
    }
}
