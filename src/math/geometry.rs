#![allow(clippy::just_underscores_and_digits)]

use crate::math::numerical::Numerical;
use std::ops::{Add, Sub};

pub struct SidesTriangle {
    sides: [u64; 3],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T, const N: usize> {
    dims: usize,
    coordinates: [T; N],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T, const N: usize> {
    dims: usize,
    coordinates: [T; N],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon<T, const N: usize> {
    pub vertices: Vec<Point<T, N>>,
}

impl <T, const N: usize> Point<T, N>
where 
    T: Numerical
{
    pub fn new(xv: Vec<T>) -> Self {
        Point {
            dims: N,
            coordinates: xv.try_into().unwrap(),
        }
    }
}

impl<T, const N: usize> Sub for Point<T, N> 
where 
    T: Numerical 
{
    type Output = Vector<T, N>;

    fn sub(self, other: Self) -> Vector<T, N> {
        let dims = N;
        let coordinates: [T; N] = 
            self.coordinates.iter().zip(other.coordinates.iter()).map(|(&x, &y)| x - y).collect::<Vec<T>>().try_into().unwrap();

        Vector {
            dims,
            coordinates,
        }
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where 
    T: Numerical
{
    type Output = Vector<T, N>;

    fn add(self, other: Vector<T, N>) -> Vector<T, N> {
        let dims = N;
        let coordinates = self.coordinates.iter().zip(other.coordinates.iter()).map(|(&x, &y)| x + y).collect::<Vec<T>>().try_into().unwrap();

        Vector {
            dims,
            coordinates,
        }
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where 
    T: Numerical
{
    type Output = Vector<T, N>;

    fn sub(self, other: Vector<T, N>) -> Vector<T, N> {
        let dims = N;
        let coordinates = self.coordinates.iter().zip(other.coordinates.iter()).map(|(&x, &y)| x - y).collect::<Vec<T>>().try_into().unwrap();

        Vector {
            dims,
            coordinates,
        }
    }
}

impl<T, const N: usize> Polygon<T, N>
where 
    T: Numerical 
{
    fn new(points: Vec<Point<T, N>>) -> Self {
        Polygon {
            vertices: points,
        }
    }
}

impl<T> Polygon<T, 2>
where 
    T: Numerical
{
    pub fn contains(self, point: Point<T, 2>) -> bool {
        let _0 = T::LFusize(0);

        let edges = self.vertices.iter().zip(self.vertices.iter().cycle().skip(1));
        let mut ret : bool = true;
        for (&p1, &p2) in edges {
            let triangle = Polygon::<T, 2>::new(vec![p1, p2, point]);
            ret &= signed_triangle_area(triangle) <= _0;
        }
        ret
    }
}

pub fn cross<T: Numerical>(a: Vector<T, 2>, b: Vector<T, 2>) -> T {
    a.coordinates[0] * b.coordinates[1] - a.coordinates[1] * b.coordinates[0]
}

pub fn signed_triangle_area<T: Numerical>(triangle: Polygon<T, 2>) -> T {
    let p0 = triangle.vertices[0];
    let p1 = triangle.vertices[1];
    let p2 = triangle.vertices[2];

    cross(p1 - p0, p2 - p1) / T::LFusize(2)
}


impl SidesTriangle {
    pub fn new(a: u64, b: u64, c: u64) -> Self {
        let mut sides = [a, b, c];
        sides.sort();
        SidesTriangle { sides }
    }

    pub fn area_heron(&self) -> f64 {
        let s: f64 = self.sides.iter().map(|&x| x as f64).sum::<f64>() / 2.0;
        (self.sides.iter().map(|&x| s - x as f64).product::<f64>() * s).sqrt()
    }

    pub fn is_almost_equlateral(&self) -> bool {
        let mut side = self.sides[0];
        let mut base = self.sides[2];
        if self.sides[1] == self.sides[2] {
            side = self.sides[1];
            base = self.sides[0];
        }
        if !base.is_multiple_of(2) {
            return false;
        }

        let hb = base / 2;
        let med2 = (side - hb) * (side + hb);
        let med = (med2 as f64).sqrt().floor() as u64;

        if med * med == med2 {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_signed_triangle_area() {
        let vertices = vec![
            Point::<f64, 2> {
                dims: 2,
                coordinates: [0.0, 0.0],
            },
            Point::<f64, 2> {
                dims: 2,
                coordinates: [0.0, 1.0],
            },
            Point::<f64, 2> {
                dims: 2,
                coordinates: [1.0, 0.0],
            }
        ];
        let triangle = Polygon::<f64, 2> {
            vertices,
        };
        assert_eq!(signed_triangle_area(triangle), -0.5);
    }

    #[test]
    pub fn test_contains() {
        let vertices = vec![
            Point::<f64, 2> {
                dims: 2,
                coordinates: [0.0, 10.0],
            },
            Point::<f64, 2> {
                dims: 2,
                coordinates: [10.0, -10.0],
            },
            Point::<f64, 2> {
                dims: 2,
                coordinates: [-10.0, -10.0],
            }
        ];
        let triangle = Polygon::<f64, 2> {
            vertices,
        };
        let point = Point::<f64, 2> {
            dims: 2,
            coordinates: [0.0, 0.0],
        };
        assert!(triangle.contains(point));
    }
}
