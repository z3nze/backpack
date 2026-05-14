use backpack::read;
use backpack::math::geometry::Point;
use std::io::Read;

fn main() {
    let mut result = 0;
    for _ in 0 .. 1000 {
        let s : String = read!();
        // let points = s.split(',')
        //     .map(|x| x.parse::<f64>())
        //     .collect::<Vec<_>>()
        //     .chunks(2)
        //     .map(|x| Point::<f64, 2>::new(vec![]))
        //     .collect();
    }
}
