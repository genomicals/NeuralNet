use rand::Rng;
use libm;

use crate::errors::NeuralNetError;

/// Neural network struct
pub struct NeuralNetwork {
    /// The shape of the neural network
    shape: Vec<usize>,
    /// Weights and biases for every perceptron
    weights: Vec<f32>,
    /// Largest layer size
    max_size: usize,
}
impl NeuralNetwork {

    /// Create a new NeuralNetwork with randomized weights
    pub fn new() -> Self {
        let shape = vec![32, 64, 64, 170];
        let weights: Vec<f32> = [(); 17322].map(|_| rand::thread_rng().gen_range(-2.0..=2.0)).to_vec();
        NeuralNetwork {shape, weights, max_size: 170}
    }


    /// Create a new NeuralNetwork with the provided weights
    pub fn with_weights(shape: Vec<usize>, weights: Vec<f32>) -> Self {
        let max_size = shape.iter().max().unwrap_or_else(|| &0).to_owned();
        NeuralNetwork {shape, weights, max_size}
    }


    /// Create a new NeuralNetwork from the provided genome
    pub fn from_genome(genome: &[f32; 17323]) -> Self {
        let weights: Vec<f32> = Vec::from(&genome[..17322]);
        let shape = vec![32, 64, 64, 170];
        NeuralNetwork {shape, weights, max_size: 170}

    }


    /// Run the input through the neural network, consider stripping for runtime reasons
    pub fn calculate(&self, input: &Vec<f32>) -> Result<Vec<f32>, NeuralNetError> {
        if input.len() != self.shape[0] {
            return Err(NeuralNetError::InvalidInputSize);
        }

        let mut mat0: Vec<f32> = vec![0.0; self.max_size];
        let mut mat1: Vec<f32> = input.clone(); //assumes the input is the right size
        mat1.extend(vec![0.0; self.max_size-self.shape[0]].iter()); //make the input the right size

        // calculate forward propagation
        let mut layer_offset = 0; //offset for each layer's weights from the start of the vector
        let mut weight_size; //size of one perceptron's inputs with bias
        let mut weight_offset; //location of current weights

        for n in 1..self.shape.len() { //for each layer
            std::mem::swap(&mut mat0, &mut mat1); //swap the two vectors
            weight_size = self.shape[n-1]+1;
            for i in 0..self.shape[n] { //for each perceptron
                mat1[i] = 0.0;
                weight_offset = layer_offset + i*weight_size;
                for j in 0..weight_size-1 { //for each input
                    mat1[i] += mat0[j] * self.weights[weight_offset + j]; //apply weight to input
                }

                mat1[i] += self.weights[weight_offset + weight_size - 1]; //apply bias
                mat1[i] = libm::tanh(mat1[i] as f64) as f32; //apply tanh normalization                                                                          
            }
            layer_offset += self.shape[n-1] * (self.shape[n]+1);
        }

        Ok(mat1)
    }


    /// Set the weights
    pub fn set_weights(&mut self, weights: Vec<f32>) -> Result<(), NeuralNetError> {
        //if self.shape[1..].iter().map(|x| x+1).product::<usize>() * self.shape[0] != weights.len() {
        //    return Err(NeuralNetError::InvalidWeightSize);
        //}
        //if self.shape[1..].iter().fold(self.shape[0], |accum, x| (accum+1) * x) != weights.len() {
        //    return Err(NeuralNetError::InvalidWeightSize);
        //} 
        self.weights = weights;
        Ok(())
    }
}

