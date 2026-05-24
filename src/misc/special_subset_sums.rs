fn sat(vs: &[usize]) -> bool {
    let n = vs.len();
    let params: Vec<(usize, usize)> = (0 .. (1 << n))
        .map(|mask| {
            vs.iter().enumerate().fold((0, 0), |(count, sum), (bit_i, v)| {
                let bit = (mask >> bit_i) & 1;
                (count + bit, sum + v * bit)
            })
        })
        .collect::<Vec<_>>();

    params.iter().enumerate()
        .all(|(i, p1)| {
            params.iter().enumerate()
                .all(|(j, p2)| {
                    (i == j) || (p1.1 != p2.1 && (p1.0 == p2.0 || ((p1.0 > p2.0) == (p1.1 > p2.1))))
                })
        })
}


pub fn special_sum_sets(n: usize) -> Vec<usize> {
    let candidates = (1 ..= n)
        .fold(vec![vec![]; 1], |acc: Vec<Vec<usize>>, _| {
            acc.iter()
                .flat_map(|set: &Vec<usize>| {
                    let next_val: usize = *set.last().unwrap_or(&1);
    
                    (next_val ..= 50)
                        .fold(Vec::new(), |mut set_expansions: Vec<Vec<usize>>, inserted_val| {
                            let mut expansion = set.clone();
                            expansion.push(inserted_val);
                            if sat(&expansion) {
                                set_expansions.push(expansion);
                            }
                            set_expansions
                        })
                })
                .collect()
        });

    candidates.iter().min_by_key(|x| x.iter().sum::<usize>()).unwrap().to_vec()
}
