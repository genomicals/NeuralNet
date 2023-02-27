use pyo3::prelude::*;
use rand::Rng;

use crate::errors::NeuralNetError;


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
    pub fn calculate(&self, input: &[i8]) -> Result<[f32; 170], NeuralNetError> {
        if input.len() != INPUT_SIZE {
            return Err(NeuralNetError::InvalidInputSize);
        } else if self.weights.len() != WEIGHT_SIZE {
            return Err(NeuralNetError::InvalidWeightSize);
        }
        let mut output: [f32; 170] = [0.0; 170];

        




        Ok(output)
    }

    /// Set the weights
    pub fn set_weights(&mut self, weights: [f32; WEIGHT_SIZE]) -> Result<(), NeuralNetError> {
        if weights.len() != WEIGHT_SIZE {
            Err(NeuralNetError::InvalidWeightSize)
        } else {
            self.weights = weights;
            Ok(())
        }
    }
}

