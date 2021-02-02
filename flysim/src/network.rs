use rand::{Rng as _, RngCore};

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

/// Rectified Linear Unit (ReLU).
fn relu(x: f32) -> f32 {
    x.max(0.0)
}

impl Network {
    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        Self {
            layers: layers
                .windows(2)
                .map(|window| Layer::random(window[0].neurons, window[1].neurons))
                .collect(),
        }
    }

    /// Forward-propagation pass.
    pub fn forward(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.forward(inputs))
    }
}

impl Layer {
    fn random(inputs: usize, outputs: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            neurons: (0..outputs)
                .map(|_| Neuron::random(&mut rng, inputs))
                .collect(),
        }
    }

    fn forward(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.forward(&inputs))
            .collect()
    }
}

impl Neuron {
    fn random(rng: &mut dyn RngCore, inputs: usize) -> Self {
        Self {
            bias: rng.gen_range(-1.0..=1.0),
            weights: (0..inputs).map(|_| rng.gen_range(-1.0..=1.0)).collect(),
        }
    }

    fn forward(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = self
            .weights
            .iter()
            .zip(inputs)
            .map(|(w, i)| w * i)
            .sum::<f32>()
            + self.bias;

        relu(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng as _;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn neuron_random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
        );
    }

    #[test]
    fn neuron_forward() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        approx::assert_relative_eq!(neuron.forward(&[10.0, -10.0]), 0.0);

        approx::assert_relative_eq!(
            neuron.forward(&[-1.0, 2.0]),
            (-0.3 * -1.0) + (0.8 * 2.0) + 0.5
        );
    }
}
