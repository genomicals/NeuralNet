use rand::Rng;

/// Neural network struct
pub struct NeuralNetwork {
    /// Weights and biases for every perceptron
    weights: Vec<f32>, //size of 17322
}
impl NeuralNetwork {
    /// Create a new NeuralNetwork with randomized weights
    pub fn new() -> Self {
        let weights: Vec<f32> = (0..17322)
            .map(|_| rand::thread_rng().gen_range(-2.0..=2.0))
            .collect();
        NeuralNetwork { weights }
    }

    /// Create a new NeuralNetwork with the provided weights
    pub fn with_weights(weights: Vec<f32>) -> Self {
        NeuralNetwork { weights }
    }

    /// Create a new NeuralNetwork from the provided genome
    pub fn from_genome(genome: &Vec<f32>) -> Self {
        //expects size of 17322
        let mut weights: Vec<f32> = vec![0.0; 17322];
        weights.copy_from_slice(&genome[..17322]); //copy weights from genome into our neural network
        NeuralNetwork::with_weights(weights)
    }

    /// Run the input through the neural network, consider stripping for runtime reasons
    pub fn calculate(&self, input: Vec<f32>) -> Vec<f32> {
        //expects input size of 32, returns size of 170
        let mut output: Vec<f32> = vec![0.0; 170]; //used for first layer and output
        let mut internal: Vec<f32> = vec![0.0; 64]; //used for second layer

        // first layer
        for i in 0..64 {
            //for each perceptron
            for j in 0..32 {
                //for each input
                output[i] += input[j] * self.weights[i * 33 + j]; //apply weight to input
            }
            output[i] += self.weights[i * 33 + 32]; //apply bias
            output[i] = output[i].tanh(); //apply tanh normalization
        }

        // second layer
        for i in 0..64 {
            //for each perceptron
            for j in 0..64 {
                //for each input
                internal[i] += output[j] * self.weights[2112 + i * 65 + j] //apply weight to input
            }
            internal[i] += self.weights[2112 + i * 65 + 64]; //apply bias
            internal[i] = internal[i].tanh(); //apply tanh normalization
        }

        //output layer
        for i in 0..170 {
            //for each perceptron
            output[i] = 0.0; //reset the element after being used for layer 1
            for j in 0..64 {
                //for each input
                output[i] += internal[j] * self.weights[6272 + i * 65 + j] //apply weight to input
            }
            output[i] += self.weights[6272 + i * 65 + 64]; //apply bias
            output[i] = output[i].tanh() * 0.5 + 1.0; //apply sigmoid normalization, consider tanh for runtime reasons
        }

        output
    }
}
