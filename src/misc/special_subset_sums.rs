pub fn is_special_sum_set(vs: &[usize]) -> bool {
    let n = vs.len();
    (0..(1 << n))
        .map(|mask: usize| {
            let b_sum: usize = vs
                .iter()
                .enumerate()
                .filter_map(|(bit_i, v)| {
                    if (mask >> bit_i) & 1 == 1 {
                        Some(v)
                    } else {
                        None
                    }
                })
                .sum();
            let b_count: usize = mask.count_ones() as usize;

            if b_count == 0 {
                return true;
            }

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
                        .filter_map(|(bit_i, v)| {
                            if (inner_mask >> bit_i) & 1 == 1 {
                                Some(v)
                            } else {
                                None
                            }
                        })
                        .sum();
                    let c_count = inner_mask.count_ones() as usize;

                    if c_count == 0 {
                        return true;
                    }

                    if b_sum == c_sum {
                        return false;
                    }

                    if b_count > c_count && b_sum < c_sum {
                        return false;
                    }

                    return true;
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
