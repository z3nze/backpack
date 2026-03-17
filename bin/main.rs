use std::num::NonZeroUsize;

use backpack::math::geometry::Triangle;

fn main() {
    let mut base = 4;
    let mut res = 0;
    let lim: usize = 1e9 as usize;
    loop {
        let lside = NonZeroUsize::new(base - 1).unwrap();
        let gside = NonZeroUsize::new(base + 1).unwrap();
        let c1 = Triangle::new(lside, lside, NonZeroUsize::new(base).unwrap());
        let c2 = Triangle::new(gside, gside, NonZeroUsize::new(base).unwrap());
        let p1 = lside.get() * 2 + base;
        let p2 = gside.get() * 2 + base;

        if p1 <= lim && c1.is_almost_equlateral() {
            res += p1;
        }
        if p2 <= lim && c2. is_almost_equlateral() {
            res += p2;
        }

        if p1 > lim && p2 > lim {
            break;
        }

        base += 2;
    }
    println!("{res}");
}
