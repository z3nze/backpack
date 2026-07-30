#[derive(Debug, Clone)]
pub struct Edge {
    from: usize,
    to: usize,
    weight: Option<usize>,
}

impl Edge {
    pub fn new(from: usize, to: usize) -> Edge {
        Edge {
            from: from,
            to: to,
            weight: None,
        }
    }

    pub fn new_weighted(from: usize, to: usize, weight: usize) -> Edge {
        Edge {
            from: from,
            to: to,
            weight: Some(weight),
        }
    }

    pub fn from(&self) -> usize {
        self.from
    }

    pub fn to(&self) -> usize {
        self.to
    }

    pub fn weight(&self) -> Option<usize> {
        self.weight
    }
}
