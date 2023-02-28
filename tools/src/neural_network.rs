use rand::Rng;
use libm;


const INPUT_SIZE: usize = 32;
const WEIGHT_SIZE: usize = 17322; //layers: 32, 64, 64, 170;   33*64 + 65*64 + 65*170 = 17322


/// Neural network struct
pub struct NeuralNetwork {
    /// Weights and biases for every perceptron
    weights: [f32; WEIGHT_SIZE],
}
impl NeuralNetwork {

    /// Create a new NeuralNetwork with randomized weights
    pub fn new() -> Self {
        let weights: [f32; WEIGHT_SIZE] = [(); WEIGHT_SIZE].map(|_| rand::thread_rng().gen_range(-2.0..=2.0));
        NeuralNetwork {weights}
    }


    /// Create a new NeuralNetwork with the provided weights
    pub fn with_weights(weights: [f32; WEIGHT_SIZE]) -> Self {
        NeuralNetwork {weights}
    }


    /// Run the input through the neural network, consider stripping for runtime reasons
    pub fn calculate(&self, input: &[f32; INPUT_SIZE]) -> [f32; 170] {
        let mut output: [f32; 170] = [0.0; 170]; //used for first layer and output
        let mut internal: [f32; 64] = [0.0; 64]; //used for second layer

        // first layer
        for i in 0..64 { //for each perceptron
            for j in 0..32 { //for each input
                output[i] += input[j] * self.weights[i*33 + j]; //apply weight to input
            }
            output[i] += self.weights[i*33 + 32]; //apply bias
            output[i] = libm::tanh(output[i] as f64) as f32; //apply tanh normalization
        } 

        // second layer
        for i in 0..64 { //for each perceptron
            for j in 0..64 { //for each input
                internal[i] += output[j] * self.weights[2112 + i*65 + j] //apply weight to input
            }
            internal[i] += self.weights[2112 + i*65 + 64]; //apply bias
            internal[i] = libm::tanh(output[i] as f64) as f32; //apply tanh normalization
        }

        //output layer
        for i in 0..170 { //for each perceptron
            output[i] = 0.0; //reset the element after being used for layer 1
            for j in 0..64 { //for each input
                output[i] += internal[j] * self.weights[6272 + i*65 + j] //apply weight to input
            }
            output[i] += self.weights[6272 + i*65 + 64]; //apply bias
            output[i] = (libm::tanh(output[i] as f64)*0.5 + 1.0) as f32; //apply sigmoid normalization, consider tanh for runtime reasons
        }

        output
    }


    /// Set the weights
    pub fn set_weights(&mut self, weights: [f32; WEIGHT_SIZE]) {
        self.weights = weights;
    }
}

