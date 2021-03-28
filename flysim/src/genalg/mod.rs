pub mod crossover;
pub mod selection;

use rand::RngCore;
use std::iter::{FromIterator, IntoIterator, Iterator};
use std::ops::Index;

pub struct GeneticAlgorithm<S: selection::Method, C: crossover::Method> {
    selection: S,
    crossover: C,
}

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl<S: selection::Method, C: crossover::Method> GeneticAlgorithm<S, C> {
    pub fn new(selection: S, crossover: C) -> Self {
        Self {
            selection,
            crossover,
        }
    }

    pub fn evolve<I: Individual>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection.select(rng, population);
                let parent_b = self.selection.select(rng, population);
                let child =
                    self.crossover
                        .crossover(rng, parent_a.chromosome(), parent_b.chromosome());

                todo!("mutation")
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
