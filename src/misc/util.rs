pub fn prod<'a, T>(xs: &'a Vec<T>, ys: &'a Vec<T>) -> Vec<(&'a T, &'a T)> {
    let mut res: Vec<(&T, &T)> = Default::default();
    res.reserve_exact(xs.len() * ys.len());
    for x in xs {
        for y in ys {
            res.push((x, y));
        }
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_prod() {
        let a: Vec<i64> = vec![10, 20, 30];
        let b: Vec<i64> = vec![50, 60, 70];

        let expect: Vec<(i64, i64)> = vec![(10, 50), (10, 60), (10, 70), (20, 50), (20, 60), (20, 70), (30, 50), (30, 60), (30, 70)];

        assert_eq!(prod(&a, &b).iter().map(|(x, y)| (**x, **y)).collect::<Vec<(i64, i64)>>(), expect);
    }
}
