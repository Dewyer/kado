use rand::prelude::*;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use rand_distr::{Distribution, Uniform};

pub struct SeededRng {
    rng: StdRng,
}

impl SeededRng {
    pub fn new(seed: i64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed as u64),
        }
    }

    pub fn skip_numbers(&mut self, to_skip: usize) {
        for _ in 0..to_skip {
            self.gen_range(0, 1);
        }
    }

    pub fn gen_range(&mut self, low: usize, high: usize) -> usize {
        let unf = Uniform::new(low, high);

        unf.sample(&mut self.rng)
    }
}