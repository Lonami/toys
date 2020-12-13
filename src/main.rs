use oorandom::Rand32;
use std::sync::Mutex;

thread_local!(static RNG: Mutex<Rand32> = Mutex::new(Rand32::new(0)));

/// Sigmoid activation function, also known as the logistic function.
///
/// It can take any input value and produce a number between 0 and 1 on an S-curve.
fn sigmoid_transfer(activation: f32) -> f32 {
    1.0 / (1.0 + f32::exp(-activation))
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
    bias: f32,
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

        sigmoid_transfer(activation)
    }
}

fn main() {
    let mut network = Network::new(2, 1, 2);
    let output = network.forward_propagate(&[1.0, 0.0]);
    dbg!(output);
}
