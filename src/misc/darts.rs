use std::iter::once;

use crate::misc::util::prod;

pub fn checkout_count(n: usize) -> u64 {
    let sectors = (1..=20)
        .flat_map(|x| (1..=3).map(move |i| (x, i)))
        .chain(once(25).flat_map(|x| (1..=2).map(move |i| (x, i))))
        .collect::<Vec<(usize, usize)>>();

    let mut cnt: Vec<u64> = vec![0; n + 1];

    sectors
        .iter()
        .filter_map(|x| if x.1 == 2 && x.0 * x.1 <= n { Some(x.0 * x.1) } else { None })
        .for_each(|x| cnt[x] += 1);

    prod(&sectors, &sectors)
        .filter_map(|(x, y)| {
            let val = x.0 * x.1 + y.0 * y.1;
            if y.1 == 2 && val <= n { Some(val) } else { None }
        })
        .for_each(|val| cnt[val] += 1);

    for (i, f) in sectors.iter().enumerate() {
        for m in sectors.iter().skip(i) {
            for l in sectors.iter() {
                if l.1 != 2 {
                    continue;
                }
                let val = f.0 * f.1 + m.0 * m.1 + l.0 * l.1;
                if val <= n {
                    cnt[val] += 1;
                }
            }
        }
    }

    cnt.iter().sum()
}
