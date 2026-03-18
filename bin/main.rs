use std::num::NonZeroUsize;

use backpack::math::geometry::Triangle;

fn main() {
    let mut base = 4;
    let mut res = 0;
    let lim: u64 = 1e9 as u64;
    loop {
        let lside =  base - 1;
        let gside = base + 1;
        let c1 = Triangle::new(lside, lside, base);
        let c2 = Triangle::new(gside, gside, base);
        let p1 = lside * 2 + base;
        let p2 = gside * 2 + base;

        if p1 <= lim && c1.is_almost_equlateral() {
            res += p1;
        }
        if p2 <= lim && c2. is_almost_equlateral() {
            res += p2;
        }

        if p1 > lim && p2 > lim {
            break;
        }

        base += 1;
    }
    println!("{res}");
}
