//! Set $A$ is called a special sum set if it satisfies following properties:
//!
//! For every $B \in A, B \neq \varnothing$ and $C \in A, C \neq \varnothing$
//! and $B \cap C = \varnothing$:
//! 1. $\sum B \neq \sum C$
//! 2. if $B$ contains more elements than $C$ then $\sum B > \sum C$

/// Checks that given set is a special sum set.
pub fn is_special_sum_set(vs: &[usize]) -> bool {
    let n = vs.len();
    (0..(1 << n))
        .map(|mask: usize| {
            let b_sum: usize = vs
                .iter()
                .enumerate()
                .fold(0, |acc, (bit_i, v)| acc + ((mask >> bit_i) & 1) * v);
            let b_count: usize = mask.count_ones() as usize;

            let rem = vs
                .iter()
                .enumerate()
                .filter_map(|(bit_i, &v)| {
                    if (mask >> bit_i) & 1 == 0 {
                        Some(v)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let m = n - b_count;

            (0..(1 << m))
                .map(|inner_mask: usize| {
                    let c_sum: usize = rem
                        .iter()
                        .enumerate()
                        .fold(0, |acc, (bit_i, v)| acc + ((inner_mask >> bit_i) & 1) * v);
                    let c_count = inner_mask.count_ones() as usize;

                    let subsets_empty = b_count == 0 && c_count == 0;
                    let property_1 = b_sum != c_sum;
                    let property_2 = b_count <= c_count || b_sum > c_sum;

                    return subsets_empty || (property_1 && property_2);
                })
                .all(|x| x)
        })
        .all(|x| x)
}

pub fn find_special_sum_set(n: usize) -> Vec<usize> {
    let candidates = (1..=n).fold(vec![vec![]; 1], |acc: Vec<Vec<usize>>, _| {
        acc.iter()
            .flat_map(|set: &Vec<usize>| {
                let next_val: usize = *set.last().unwrap_or(&0) + 1;
                let ubound = if set.len() >= 2 {
                    set.iter().take(2).sum()
                } else {
                    50
                };

                (next_val..=ubound).fold(
                    Vec::new(),
                    |mut set_expansions: Vec<Vec<usize>>, inserted_val| {
                        let mut expansion = set.clone();
                        expansion.push(inserted_val);
                        if is_special_sum_set(&expansion) {
                            set_expansions.push(expansion);
                        }
                        set_expansions
                    },
                )
            })
            .collect()
    });

    candidates
        .iter()
        .min_by_key(|x| x.iter().sum::<usize>())
        .unwrap()
        .to_vec()
}

// Assume that set $A% contains n elements and already satisfies property 2.
// This function calculates how many times property 1 needs to be non-trivially checked.
pub fn count_property_1_checks(n: usize) -> usize {
    let mut res = 0;
    for mask in 0..(1 << n) {
        let keep: Vec<usize> = (0..n).filter(|i| (mask >> i) & 1 == 1).collect();
        let m = keep.len();
        for inner_mask in 0..(1 << m) {
            let (b, c): (Vec<usize>, Vec<usize>) = (0..m).partition(|i| (inner_mask >> i) & 1 == 1);
            if b.len() != c.len() {
                continue;
            }
            let satbc = b.iter().zip(c.iter()).all(|(x, y)| x < y);
            let satcb = c.iter().zip(b.iter()).all(|(x, y)| x < y);
            let sat = satbc | satcb;
            if !sat {
                res += 1;
            }
        }
    }
    return res / 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_special_sum_set() {
        assert!(is_special_sum_set(&vec![3, 5, 6, 7]));
        assert!(!is_special_sum_set(&vec![1, 2, 3, 4]));
    }

    #[test]
    pub fn test_special_sum_sets() {
        assert_eq!(find_special_sum_set(1), vec![1]);
        assert_eq!(find_special_sum_set(2), vec![1, 2]);
        assert_eq!(find_special_sum_set(3), vec![2, 3, 4]);
        assert_eq!(find_special_sum_set(4), vec![3, 5, 6, 7]);
    }
}
