use oorandom::Rand32;
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
    fn train(&mut self, epochs: usize, lrate: f32, dataset: &Vec<Vec<f32>>) {
        let output_count = self.layers[self.layers.len() - 1].neurons.len();
        (0..epochs).for_each(|epoch| {
            let error = dataset
                .iter()
                .map(|row| {
                    let (inputs, output) = (&row[..row.len() - 1], row[row.len() - 1] as usize);
                    let outputs = self.forward_propagate(inputs);
                    let mut expected = vec![0.0; output_count];
                    expected[output] = 1.0;
                    self.backward_propagate_error(&expected);
                    self.update_weights(inputs, lrate);

                    expected
                        .iter()
                        .zip(outputs.iter())
                        .map(|(e, o)| (e - o).powi(2))
                        .sum::<f32>()
                })
                .sum::<f32>();

            eprintln!(
                "> epoch {:.3}, lrate {:.3}, error {:.3}",
                epoch, lrate, error
            );
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

fn main() {
    let dataset = vec![
        vec![2.7810836, 2.550537003, 0.0],
        vec![1.465489372, 2.362125076, 0.0],
        vec![3.396561688, 4.400293529, 0.0],
        vec![1.38807019, 1.850220317, 0.0],
        vec![3.06407232, 3.005305973, 0.0],
        vec![7.627531214, 2.759262235, 1.0],
        vec![5.332441248, 2.088626775, 1.0],
        vec![6.922596716, 1.77106367, 1.0],
        vec![8.675418651, -0.242068655, 1.0],
        vec![7.673756466, 3.508563011, 1.0],
    ];
    let mut network = Network::new(2, 2, 2);
    network.train(20, 0.5, &dataset);
    dbg!(network);
}
