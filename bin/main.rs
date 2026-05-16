use backpack::read;
use backpack::math::geometry::{Point, Polygon};
use std::io::{Read, Write};

fn main() {
    let mut result = 0;
    let zero = Point::new(vec![0.0, 0.0]);
    for _ in 0 .. 1000 {
        let s : String = read!();
        let points = s.split(',')
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|x| Point::<f64, 2>::new(vec![x[0], x[1]]))
            .collect::<Vec<_>>();
        let triangle = Polygon::<f64, 2> {
            vertices: points,
        };
        if triangle.contains(zero) {
            result += 1;
        }
    }
    result += 1;
    std::io::stdout().write_all(result.to_string().as_bytes()).expect("write fail");
    std::io::stdout().flush().expect("flush fail");
}
