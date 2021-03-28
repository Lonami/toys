use super::Individual;
use rand::{Rng, RngCore};
use std::cell::RefCell;

pub trait Method {
    fn select<'a, I: Individual>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I;
}

pub struct RouletteWheel {
    cum_fitness: RefCell<Vec<f32>>,
}

impl RouletteWheel {
    pub fn new() -> Self {
        Self {
            cum_fitness: RefCell::new(Vec::new()),
        }
    }
}

impl Method for RouletteWheel {
    fn select<'a, I: Individual>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I {
        assert!(!population.is_empty());

        let mut cum_fitness = self.cum_fitness.borrow_mut();
        cum_fitness.clear();
        let max_fitness = population.iter().fold(0f32, |mut acc, x| {
            acc += x.fitness();
            cum_fitness.push(acc);
            acc
        });
        let seek = rng.gen_range(0f32..max_fitness);
        match cum_fitness.binary_search_by(|probe| probe.partial_cmp(&seek).unwrap()) {
            Ok(i) => &population[i],
            Err(i) => &population[i],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng as _;
    use rand_chacha::ChaCha8Rng;

    struct Doge {
        fitness: f32,
    }

    impl Individual for Doge {
        fn fitness(&self) -> f32 {
            self.fitness
        }
    }

    #[test]
    fn roulette() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let population = [
            Doge { fitness: 1.0 },
            Doge { fitness: 2.0 },
            Doge { fitness: 3.0 },
        ];
        let method = RouletteWheel::new();

        for &fitness in [2.0, 3.0, 3.0, 3.0, 3.0, 2.0, 3.0, 1.0].iter() {
            assert_eq!(method.select(&mut rng, &population).fitness, fitness);
        }
    }
}
