pub struct DSU {
    set: Vec<usize>,
    rank: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        DSU {
            set: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    pub fn get(&mut self, x: usize) -> usize {
        if x == self.set[x] {
            return x;
        }
        self.set[x] = self.get(self.set[x]);
        return self.set[x];
    }

    pub fn merge(&mut self, x: usize, y: usize) {
        let mut dx = self.get(x);
        let mut dy = self.get(y);

        if dx == dy {
            return;
        }

        if self.rank[dx] < self.rank[dy] {
            (dx, dy) = (dy, dx);
        }

        if self.rank[dx] == self.rank[dy] {
            self.rank[dx] += 1;
        }
        self.set[dy] = dx;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_dsu() {
        let mut dsu = DSU::new(7);

        dsu.merge(0, 1);
        dsu.merge(0, 2);
        dsu.merge(0, 3);
        dsu.merge(1, 3);

        dsu.merge(4, 5);
        dsu.merge(5, 6);

        assert!(dsu.get(0) == 0);
        assert!(dsu.get(1) == 0);
        assert!(dsu.get(2) == 0);
        assert!(dsu.get(3) == 0);

        assert!(dsu.get(4) == 4);
        assert!(dsu.get(5) == 4);
        assert!(dsu.get(6) == 4);

        dsu.merge(4, 0);

        assert!(dsu.get(0) == 4);
        assert!(dsu.get(4) == 4);
    }
}
