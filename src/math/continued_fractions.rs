pub struct QuadraticIrrational {
    _d: i64,
    _non_periodic: Vec<usize>,
    _period: Vec<usize>,
}

impl QuadraticIrrational {
    pub fn new(d: i64) -> Self {
        let non_periodic: Vec<usize> = vec![];
        let period: Vec<usize> = vec![];

        let mut a: Vec<i64> = vec![0];
        let mut b: Vec<i64> = vec![d.isqrt()];
        let mut acc: i64 = 0;
        for _ in 0..10 {
            let tb = d.isqrt() + acc;
            acc = tb - acc;
            let ta = d - acc.pow(2);
            a.push(ta);
            b.push(tb);
        }

        QuadraticIrrational{ _d: d, _non_periodic: non_periodic, _period: period }
    }
}
