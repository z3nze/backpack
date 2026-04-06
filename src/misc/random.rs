pub struct Xoshiro256ppGenerator {
    state: [u64; 4]
}

impl Xoshiro256ppGenerator {
    pub fn new(seed: u64) -> Self {
        let mut sm64g = SplitMix64Generator::new(seed);
        let t1 = sm64g.rand();
        let t2 = sm64g.rand();

        Xoshiro256ppGenerator { state: [t1, (t1 >> 32), t2, (t2 >> 32)] }
    }

    pub fn rand(&mut self) -> u64 {
        let res = (self.state[0] + self.state[3]).rotate_left(23) + self.state[0];
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        res
    }
}

pub struct SplitMix64Generator {
    state: u64
}

impl SplitMix64Generator {
    pub fn new(seed: u64) -> Self {
        SplitMix64Generator{ state: seed }
    }

    pub fn rand(&mut self) -> u64 {
        self.state += 0x9E3779B97F4A7C15;
        let mut result = self.state;
        result = (result ^ (result >> 30)) * 0xBF58476D1CE4E5B9;
        result = (result ^ (result >> 27)) * 0x94D049BB133111EB;
        result ^ (result >> 31)
    }
}
