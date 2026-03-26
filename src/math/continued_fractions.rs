pub struct QuadraticIrrational {
    _d: usize,
    _non_periodic: Vec<usize>,
    _period: Vec<usize>,
}

impl QuadraticIrrational {
    pub fn new(d: usize) -> Self {
        let non_periodic: Vec<usize> = vec![];
        let period: Vec<usize> = vec![];
        QuadraticIrrational{ _d: d, _non_periodic: non_periodic, _period: period }
    }
}
