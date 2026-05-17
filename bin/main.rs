use backpack::read;
use backpack::math::geometry::{Point, Polygon, signed_triangle_area};
use std::io::{Read};

fn main() {
    let mut result = 0;
    let zero = Point::new(vec![0.0, 0.0]);
    for _ in 0 .. 1000 {
        let s : String = read!();
        let input_points_flat = s
            .split(',')
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<_>>();

        let input_points = input_points_flat
            .chunks(2)
            .collect::<Vec<_>>();

        let mut points = input_points.iter()
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
