use oorandom::Rand32;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::sync::Mutex;

thread_local!(static RNG: Mutex<Rand32> = Mutex::new(Rand32::new(0)));

/// Sigmoid activation function, also known as the logistic function.
///
/// It can take any input value and produce a number between 0 and 1 on an S-curve.
fn sigmoid_transfer(activation: f32) -> f32 {
    1.0 / (1.0 + f32::exp(-activation))
}

fn sigmoid_derivative(output: f32) -> f32 {
    output * (1.0 - output)
}

/// A multiplayer feed-forward Neural Network (NN), which implements the backpropagation
/// algorithm. It can be used for classification and regression problems.
///
/// For classification problems, it is recommended to have one neuron per possible output, so that
/// only a single neuron is the most active (known as "one hot encoding").
#[derive(Debug)]
struct Network {
    layers: Vec<Layer>,
}

/// A single layer from a Neural Network.
///
/// Normally, the input layer is not represented, since it simply maps the actual input data.
#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

/// A single neuron.
///
/// It accepts inputs on its current layer and passes outputs to subsequent layers.
///
/// The neuron's weights are modified via backpropagation from the last layer towards the first
/// one, taking into consideration both the absolute error and the neuron's overall influence.
///
/// Backpropagating the error "trains" the neurons to behave better over time.
#[derive(Debug)]
struct Neuron {
    weights: Vec<f32>,
    /// The bias can also be seen as a weight whose input is always 1.
    bias: f32,
    /// The output value from the last activation, or 0.
    ///
    /// Needed during backpropagation.
    output: f32,
    /// Delta error after backpropagation with the expected outputs.
    ///
    /// Needed during training to update the weights.
    delta: f32,
}

impl Network {
    fn new(inputs: usize, hidden: usize, outputs: usize) -> Self {
        Self {
            layers: vec![Layer::new(inputs, hidden), Layer::new(hidden, outputs)],
        }
    }

    /// Predict which category the given inputs likely belongs to.
    fn predict_category(&mut self, inputs: &[f32]) -> usize {
        let outputs = self.forward_propagate(inputs);
        outputs
            .into_iter()
            .enumerate()
            .max_by(|(_, x0), (_, x1)| f32::partial_cmp(x0, x1).unwrap())
            .unwrap()
            .0
    }

    /// Forward-propagates the given inputs through the network.
    ///
    /// This propagates the input "signals" through the network to produce an output. It
    /// essentially lets the network make predictions for a given set of inputs.
    fn forward_propagate(&mut self, inputs: &[f32]) -> Vec<f32> {
        // All outputs from one layer become inputs to the next.
        self.layers
            .iter_mut()
            .fold(inputs.to_vec(), |inputs, layer| {
                layer
                    .neurons
                    .iter_mut()
                    .map(|neuron| neuron.activate(&inputs))
                    .collect()
            })
    }

    /// The backpropagation algorithm is named for the way in which weights are trained.
    ///
    /// The error is calculated between the expected outputs and the outputs forward propagated
    /// from the network. These errors are then propagated backward through the network from the
    /// output layer to the hidden layer, assigning blame for the error and updating weights as
    /// they go.
    ///
    /// To calculate the error, the slope from a neuron's output is used (hence the derivative of
    /// the activation function).
    ///
    /// # Panics
    ///
    /// Panics if the amount of neurons in the output layer does not match the amount of expected
    /// values.
    fn backward_propagate_error(&mut self, expected: &[f32]) {
        self.layers.iter_mut().rev().fold(
            (None, Vec::new()),
            |(last_layer, errors): (Option<&Layer>, _), layer| {
                let errors = if let Some(last_layer) = last_layer {
                    // Errors in hidden layer depend on the neurons' weights.
                    layer
                        .neurons
                        .iter_mut()
                        .zip(errors.iter())
                        .enumerate()
                        .map(|(i, (neuron, error))| {
                            neuron.apply_error_from_output(i, *error, last_layer)
                        })
                        .collect()
                } else {
                    // Calculate errors at the output layer.
                    layer
                        .neurons
                        .iter_mut()
                        .zip(expected.iter())
                        .map(|(n, e)| n.apply_error_from_expected(*e))
                        .collect::<Vec<_>>()
                };

                (Some(layer), errors)
            },
        );
    }

    fn update_weights(&mut self, inputs: &[f32], lrate: f32) {
        self.layers
            .iter_mut()
            .fold(inputs.to_vec(), |inputs, layer| {
                layer
                    .neurons
                    .iter_mut()
                    .map(|neuron| {
                        for i in 0..inputs.len() {
                            neuron.weights[i] += lrate * neuron.delta * inputs[i];
                        }
                        neuron.bias += lrate * neuron.delta;
                        neuron.output
                    })
                    .collect()
            });
    }

    /// Train the network. This is done by updating it using stochastic gradient descent.
    ///
    /// The method will run for the specified amount of epochs, updating the network with the data
    /// from the training dataset. Updates are made for each training pattern, known as "online
    /// learning". The alternative, accumulating errors across an epoch before updating the
    /// weights, is known as batch learning or batch gradient descent.
    ///
    /// The training set should be a vector containing rows of input data and the expected value
    /// as the last element.
    fn train(&mut self, epochs: usize, lrate: f32, dataset: &[Vec<f32>]) {
        let output_count = self.layers[self.layers.len() - 1].neurons[0].weights.len();
        (0..epochs).for_each(|_| {
            dataset.iter().for_each(|row| {
                let (inputs, output) = (&row[..row.len() - 1], row[row.len() - 1] as usize);
                let outputs = self.forward_propagate(inputs);
                let mut expected = vec![0.0; output_count];
                expected[output] = 1.0;
                self.backward_propagate_error(&expected);
                self.update_weights(inputs, lrate);

                let _error = expected
                    .iter()
                    .zip(outputs.iter())
                    .map(|(e, o)| (e - o).powi(2))
                    .sum::<f32>();
            });
        });
    }
}

impl Layer {
    fn new(inputs: usize, size: usize) -> Self {
        Self {
            neurons: (0..size).map(|_| Neuron::randomised(inputs)).collect(),
        }
    }
}

impl Neuron {
    /// It is good practice to initialize the neurons' weights to small random numbers. This
    /// method creates a new neuron with its weights randomised between 0 and 1.
    fn randomised(inputs: usize) -> Self {
        RNG.with(|rng| {
            let mut rng = rng.lock().unwrap();
            Self {
                weights: (0..inputs).map(|_| rng.rand_float()).collect(),
                bias: rng.rand_float(),
                output: 0.0,
                delta: 0.0,
            }
        })
    }

    #[cfg(test)]
    fn new(weights: Vec<f32>, bias: f32) -> Self {
        Self {
            weights,
            bias,
            output: 0.0,
            delta: 0.0,
        }
    }

    /// Calculates the activation of one neuron given an input.
    ///
    /// The input is either data from the input dataset, or outputs from previous layers.
    ///
    /// # Panics
    ///
    /// Panics if the amount of inputs does not match the amount of weights.
    fn activate(&mut self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        let activation = self
            .weights
            .iter()
            .zip(inputs.iter())
            .map(|(w, i)| w * i)
            .sum::<f32>()
            + self.bias;

        self.output = sigmoid_transfer(activation);
        self.output
    }

    /// Apply error from an expected output.
    fn apply_error_from_expected(&mut self, expected: f32) -> f32 {
        self.delta = (expected - self.output) * sigmoid_derivative(self.output);
        self.delta
    }

    /// Apply error from the outputs of a previous layer.
    fn apply_error_from_output(&mut self, i: usize, error: f32, last_layer: &Layer) -> f32 {
        // TODO needing the index to access the correct weights is a bit weird
        let error = last_layer
            .neurons
            .iter()
            .map(|neuron| neuron.weights[i] * error)
            .sum::<f32>();

        self.delta = error * sigmoid_derivative(self.output);
        self.delta
    }
}

fn main() -> io::Result<()> {
    // Open dataset for reading.
    let reader = BufReader::new(File::open("wheat-seeds.csv")?);

    // Load dataset into memory.
    let mut dataset = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<f32>>>();

    // Create a mapping for the possible categories to indices.
    let mapping = {
        let mut mapping = HashMap::new();
        let mut id = 0;
        dataset.iter().for_each(|row| {
            mapping
                .entry(row[row.len() - 1] as usize)
                .or_insert_with(|| {
                    let value = id;
                    id += 1;
                    value
                });
        });
        mapping
    };

    // Normalize the mapping (inputs in range [0, 1], and output with the mapping applied).
    {
        let mut limits = vec![(f32::INFINITY, f32::NEG_INFINITY); dataset[0].len() - 1];
        dataset.iter().for_each(|row| {
            row.iter()
                .zip(limits.iter_mut())
                .for_each(|(&x, (lo, hi))| {
                    *lo = lo.min(x);
                    *hi = hi.max(x);
                });
        });
        dataset.iter_mut().for_each(|row| {
            row.iter_mut()
                .zip(limits.iter())
                .for_each(|(ref mut x, (lo, hi))| **x = (**x - *lo) / (*hi - *lo));
            let i = row.len() - 1;
            row[i] = *mapping.get(&(row[i] as usize)).unwrap() as f32;
        });
    }

    let inputs = dataset[0].len() - 1;
    let hidden = 5;
    let outputs = mapping.len();

    let lrate = 0.3;
    let epochs = 500;

    let mut network = Network::new(inputs, hidden, outputs);

    // Shuffle the dataset and train with the a few items, then predict the rest.
    RNG.with(|rng| {
        let mut rng = rng.lock().unwrap();
        (0..dataset.len() * 5).for_each(|_| {
            let r = 0..dataset.len() as u32;
            dataset.swap(
                rng.rand_range(r.clone()) as usize,
                rng.rand_range(r) as usize,
            );
        });
    });

    let train = (0.9 * dataset.len() as f32) as usize;
    let predict = &dataset[train..];
    let train = &dataset[..train];

    network.train(epochs, lrate, train);
    predict.iter().for_each(|row| {
        let (inputs, expected) = (&row[..row.len() - 1], row[row.len() - 1] as usize);
        let predicted = network.predict_category(inputs);
        println!(
            "{}: Expected: {}; Got: {}",
            if predicted == expected { "OK" } else { "KO" },
            expected,
            predicted
        );
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    // Precise values from these tests were generated with the reference Python implementation.
    use super::*;

    const DELTA: f32 = 0.00000001;
    const LARGE_DELTA: f32 = 0.00001;

    /// Return a neural network with 2 input neurons, 3 hidden neurons and 2 output neurons.
    ///
    /// It was trained to output ((i0 > 1.0 / 3.0) && (i1 < 2.0 / 3.0)) as i32, and then the
    /// weights were rounded to the closest integer.
    fn get_trained_nn() -> Network {
        Network {
            layers: vec![
                Layer {
                    neurons: vec![
                        Neuron::new(vec![13.0, 4.0], -6.0),
                        Neuron::new(vec![3.0, -5.0], -1.0),
                        Neuron::new(vec![1.0, 9.0], -7.0),
                    ],
                },
                Layer {
                    neurons: vec![
                        Neuron::new(vec![-9.0, -4.0, 10.0], 4.0),
                        Neuron::new(vec![9.0, 4.0, -10.0], -5.0),
                    ],
                },
            ],
        }
    }

    /// Return a neural network that hasn't learnt any particular problem.
    fn get_nn() -> Network {
        Network {
            layers: vec![
                Layer {
                    neurons: vec![
                        Neuron::new(vec![1.0, -2.0], 3.0),
                        Neuron::new(vec![-4.0, 5.0], -6.0),
                        Neuron::new(vec![7.0, -8.0], 9.0),
                    ],
                },
                Layer {
                    neurons: vec![
                        Neuron::new(vec![1.0, -2.0, 3.0], -4.0),
                        Neuron::new(vec![-5.0, 6.0, -7.0], 8.0),
                    ],
                },
            ],
        }
    }

    #[test]
    fn test_sigmoid_transfer() {
        let pairs = [
            (-5.0, 0.0066928509242848554),
            (-1.0, 0.2689414213699951),
            (-0.5, 0.3775406687981454),
            (0.0, 0.5),
            (0.5, 0.6224593312018546),
            (1.0, 0.7310585786300049),
            (5.0, 0.9933071490757153),
        ];
        pairs.iter().for_each(|&(x, expected)| {
            assert!(
                f32::abs(sigmoid_transfer(x) - expected) < DELTA,
                format!("bad output for x = {}", x)
            );
        });
    }

    #[test]
    fn test_sigmoid_derivative() {
        let pairs = [
            (-5.0, -30.0),
            (-1.0, -2.0),
            (-0.5, -0.75),
            (0.0, 0.0),
            (0.5, 0.25),
            (1.0, 0.0),
            (5.0, -20.0),
        ];
        pairs.iter().for_each(|&(x, expected)| {
            assert!(
                f32::abs(sigmoid_derivative(x) - expected) < DELTA,
                format!("bad output for x = {}", x)
            );
        });
    }

    #[test]
    fn test_predict_category() {
        let mut network = get_trained_nn();

        assert_eq!(network.predict_category(&[0.0 / 3.0, 1.0 / 3.0]), 0);
        assert_eq!(network.predict_category(&[2.0 / 3.0, 1.0 / 3.0]), 1);
    }

    #[test]
    fn test_forward_propagate() {
        let mut network = get_trained_nn();

        let outputs = network.forward_propagate(&[2.0 / 3.0, 1.0 / 3.0]);
        assert!(f32::abs(outputs[0] - 0.0028697780623672) < DELTA);
        assert!(f32::abs(outputs[1] - 0.9922374124398134) < DELTA);
    }

    #[test]
    fn test_backward_propagate_error() {
        let mut network = get_nn();

        network.forward_propagate(&[0.0, 1.0]);
        network.backward_propagate_error(&[0.0, 1.0]);

        let pairs = [
            (network.layers[0].neurons[0].delta, -0.06688876741137309),
            (network.layers[0].neurons[1].delta, 0.0838845457008521),
            (network.layers[0].neurons[2].delta, -0.1008803239903311),
            (network.layers[1].neurons[0].delta, -0.02300232205870664),
            (network.layers[1].neurons[1].delta, 0.06344094722446822),
        ];
        pairs.iter().enumerate().for_each(|(i, &(x, expected))| {
            assert!(
                f32::abs(x - expected) < LARGE_DELTA,
                format!(
                    "bad output at i = {} (got = {}, expected {})",
                    i, x, expected
                )
            );
        });
    }

    #[test]
    fn test_activate() {
        let mut neuron = Neuron::new(vec![-1.0, 2.0], -3.0);
        assert!(f32::abs(neuron.activate(&[0.3, 0.6]) - 0.10909682119561298) < LARGE_DELTA);
    }
}
