#[derive(Debug, Clone)]
pub struct Edge {
    from: usize,
    to: usize,
    weight: Option<usize>,
    capacity: Option<usize>,
    cost: Option<usize>,
    flow: Option<usize>,
}

impl Edge {
    pub fn new(from: usize, to: usize) -> Edge {
        Edge {
            from: from,
            to: to,
            weight: None,
            capacity: None,
            cost: None,
            flow: None,
        }
    }

    pub fn new_weighted(from: usize, to: usize, weight: usize) -> Edge {
        Edge {
            from: from,
            to: to,
            weight: Some(weight),
            capacity: None,
            cost: None,
            flow: None,
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

    pub fn capacity(&self) -> Option<usize> {
        self.capacity
    }

    pub fn cost(&self) -> Option<usize> {
        self.cost
    }

    pub fn flow(&self) -> Option<usize> {
        self.flow
    }
}
