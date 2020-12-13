use oorandom::Rand32;
use std::sync::Mutex;

thread_local!(static RNG: Mutex<Rand32> = Mutex::new(Rand32::new(0)));

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
}

fn main() {
    dbg!(Network::new(2, 1, 2));
}
