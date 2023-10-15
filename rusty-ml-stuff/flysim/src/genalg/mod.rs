pub mod crossover;
pub mod mutation;
pub mod selection;

use rand::RngCore;
use std::iter::{FromIterator, IntoIterator, Iterator};
use std::ops::Index;

pub struct GeneticAlgorithm<S, C, M>
where
    S: selection::Method,
    C: crossover::Method,
    M: mutation::Method,
{
    selection: S,
    crossover: C,
    mutation: M,
}

pub trait Individual: From<Chromosome> {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl<S, C, M> GeneticAlgorithm<S, C, M>
where
    S: selection::Method,
    C: crossover::Method,
    M: mutation::Method,
{
    pub fn new(selection: S, crossover: C, mutation: M) -> Self {
        Self {
            selection,
            crossover,
            mutation,
        }
    }

    pub fn evolve<I: Individual>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection.select(rng, population);
                let parent_b = self.selection.select(rng, population);
                let mut child =
                    self.crossover
                        .crossover(rng, parent_a.chromosome(), parent_b.chromosome());
                self.mutation.mutate(rng, &mut child);
                I::from(child)
            })
            .collect()
    }
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}
