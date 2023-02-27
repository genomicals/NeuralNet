use pyo3::prelude::*;

/// Neural network struct
pub struct NeuralNetwork {
    //weights: i32,
}
impl NeuralNetwork {
    pub fn new() -> Self {
        NeuralNetwork {}
    }

    /// Run the input through the neural network
    pub fn calculate(&self, input: &[i8]) -> Vec<f32> {
        Vec::with_capacity(170)
    }
}

