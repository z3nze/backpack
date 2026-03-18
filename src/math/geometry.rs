pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn new(a: u64, b: u64, c: u64) -> Self {
        let mut sides = [a, b, c];
        sides.sort();
        Triangle { sides }
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
