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
