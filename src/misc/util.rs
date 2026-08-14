pub fn prod<'a, T, X, Y>(xs: &'a X, ys: &'a Y) -> impl Iterator<Item = (&'a T, &'a T)> + 'a
where
    T: 'a,
    &'a X: IntoIterator<Item = &'a T>,
    &'a Y: IntoIterator<Item = &'a T>,
{
    xs.into_iter()
        .flat_map(move |x| ys.into_iter().map(move |y| (x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prod() {
        let a: Vec<i64> = vec![10, 20, 30];
        let b: Vec<i64> = vec![50, 60, 70];

        let expect: Vec<(i64, i64)> = vec![
            (10, 50),
            (10, 60),
            (10, 70),
            (20, 50),
            (20, 60),
            (20, 70),
            (30, 50),
            (30, 60),
            (30, 70),
        ];

        assert_eq!(
            prod(&a, &b)
                .map(|(&x, &y)| (x, y))
                .collect::<Vec<(i64, i64)>>(),
            expect
        );
    }
}
