use super::Chromosome;
use rand::{Rng, RngCore};

pub trait Method {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

pub struct Uniform;

impl Method for Uniform {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        let parent_a = parent_a.iter();
        let parent_b = parent_b.iter();

        parent_a
            .zip(parent_b)
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng as _;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn uniform() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a = [1.0f32, 2.0, 3.0].iter().copied().collect();
        let parent_b = [-1.0f32, -2.0, -3.0].iter().copied().collect();

        let method = Uniform;
        let child = method.crossover(&mut rng, &parent_a, &parent_b);

        assert_eq!(
            child.iter().copied().collect::<Vec<_>>(),
            [-1.0f32, -2.0, 3.0]
        );
    }
}
