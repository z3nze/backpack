use backpack::math::geometry::{Point, Polygon, signed_triangle_area};
use backpack::io::console::Scanner;

fn main() {
    let mut result = 0;
    let zero = Point::new(vec![0.0, 0.0]);
    let mut cin = Scanner::default();
    for _ in 0 .. 1000 {
        let s : String = cin.read();
        let mut points = s
            .split(',')
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
            .collect::<Vec<_>>()
            .iter()
            .map(|x| Point::<f64, 2>::new(vec![x[0], x[1]]))
            .collect::<Vec<_>>();
        
        let mut triangle = Polygon::<f64, 2> {
            vertices: points.clone(),
        };
        if signed_triangle_area(&triangle) > 0.0 {
            points.reverse();
            triangle = Polygon { vertices: points };
        }
        if triangle.contains(zero) {
            result += 1;
        }
    }
    println!("{}", result);
}
