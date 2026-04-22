#![allow(clippy::just_underscores_and_digits)]

use crate::math::numerical::Numerical;

pub struct SidesTriangle {
    sides: [u64; 3],
}

pub struct Point<T, const N: usize> {
    dims: usize,
    coordinates: [T; N],
}

pub struct Polygon2D<T, const N: usize> {
    vertices: [Point<T, 2>; N],
}

impl<T, const N: usize> Polygon2D<T, N> {
    pub fn is_inside(point: Point<T, 2>) -> bool {
        unimplemented!()
    }
}

pub fn cross<T: Numerical>(a: Point<T, 2>, b: Point<T, 2>) -> T {
    a.coordinates[0] * b.coordinates[1] - a.coordinates[1] * b.coordinates[0]
}

pub fn signed_triangle_area<T: Numerical>(triangle: Polygon2D<T, 3>) -> T {
    let _0 = T::try_from(0).unwrap();
    _0
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

pub struct PointsTriangle {
    vertices: [(f64, f64); 3],
}
