use super::Chromosome;
use rand::{Rng, RngCore};

pub trait Method {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

pub struct Gaussian {
    /// 0 for no genes changing, 1 for all genes changing.
    chance: f32,
    /// Maximum (and minimum, with inverted sign) change.
    coeff: f32,
}

impl Gaussian {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);
        Self { chance, coeff }
    }
}

impl Method for Gaussian {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            if rng.gen_bool(self.chance as _) {
                let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}
