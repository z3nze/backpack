use backpack::math::geometry::{Point, Polygon, signed_triangle_area};
use backpack::io::console::Scanner;

fn string_to_points(s: &str) -> Vec<Point<f64, 2>> {
    s
    .split(',')
    .map(|x| x.parse::<f64>().unwrap())
    .collect::<Vec<_>>()
    .chunks_exact(2)
    .map(|x| Point::<f64, 2>::new(vec![x[0], x[1]]))
    .collect::<Vec<_>>()
}

fn points_to_polygon(points: &mut [Point<f64,2>]) -> Polygon<f64, 2> {
    let mut triangle = Polygon::<f64, 2> {
        vertices: points.to_owned(),
    };
    if signed_triangle_area(&triangle) > 0.0 {
        points.reverse();
        triangle = Polygon { vertices: points.to_vec() };
    }
    triangle
}

fn main() {
    let zero = Point::new(vec![0.0, 0.0]);
    let mut cin = Scanner::default();
    let result = (0..1000)
        .map(|_| cin.read::<String>())
        .map(|s| string_to_points(&s))
        .map(|mut points| points_to_polygon(&mut points))
        .filter(|triangle| triangle.contains(zero))
        .count();
    println!("{}", result);
}
