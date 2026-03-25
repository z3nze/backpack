pub struct SegmentTree {
    _buf: Vec<i64>
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        let buf: Vec<i64> = vec![0; 4 * n];
        SegmentTree { _buf: buf }
    }

    pub fn add(&mut self, _l: usize, _r: usize, _val: usize) {
    }
}
