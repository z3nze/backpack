use backpack::math::polynomials::LagrangeInterpolation;

fn u_gen(n: f64) -> f64 {
    let mut res: f64 = 1.0;
    let mut p = 1.0;
    for _ in 1..=10 {
        p *= -n;
        res += p;
    }
    res
}

fn main() {
    let mut points: Vec<(f64, f64)> = vec![];

    let mut res: f64 = 0.0;
    for i in 1..=10 {
        points.push((i as f64, u_gen(i as f64)));
        let li = LagrangeInterpolation::new(&points);
        res += li.P((i + 1) as f64);
    }

    println!("{}", res);
}
