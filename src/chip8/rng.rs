// https://en.wikipedia.org/wiki/Linear_congruential_generator
// https://en.wikipedia.org/wiki/Lehmer_random_number_generator

// RNG(n+1) = (A * RNG(n) + C) mod M

const RNG_A: u32 = 48271;
const RNG_C: u32 = 1;
const RNG_M: u32 = 2147483647;

pub struct RNG {
    seed: u32,
}

impl RNG {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn next(&mut self) -> u8 {
        self.seed = RNG_A.overflowing_mul(self.seed).0.overflowing_add(RNG_C).0 % RNG_M;
        (self.seed % 256) as u8
    }
}
