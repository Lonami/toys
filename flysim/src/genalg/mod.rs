pub mod selection;

pub struct GeneticAlgorithm;

pub trait Individual {
    fn fitness(&self) -> f32;
}

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn evolve<I: Individual>(population: &[I]) -> &I {
        assert!(!population.is_empty());
        todo!()
    }
}
