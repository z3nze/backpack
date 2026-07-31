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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_edge() {
        let new_non_weighted = Edge::new(0, 1);
        assert_eq!(new_non_weighted.from(), 0);
        assert_eq!(new_non_weighted.to(), 1);
        assert_eq!(new_non_weighted.weight(), None);
        assert_eq!(new_non_weighted.capacity(), None);
        assert_eq!(new_non_weighted.flow(), None);

        let new_weighted = Edge::new_weighted(3, 4, 10);
        assert_eq!(new_weighted.from(), 3);
        assert_eq!(new_weighted.to(), 4);
        assert_eq!(new_weighted.weight(), Some(10));
        assert_eq!(new_weighted.capacity(), None);
        assert_eq!(new_weighted.flow(), None);
    }
}
