use rand::Rng;
use pyo3::prelude::*;
use crate::neural_network::NeuralNetwork;

/// AI struct
//#[pyclass]
pub struct AI {
    neuralnet: NeuralNetwork,
    genome: [f32; 17323], //weights + biases + mutability
    //#[pyo3(get, set)]
    //test_int: usize,
}
//#[pymethods]
impl AI {
    //#[new]
    /// Create an AI with a randomized genome
    pub fn new() -> Self {
        let genome = [(); 17323].map(|_| rand::thread_rng().gen_range(-1.0..=1.0));
        AI {
            neuralnet: NeuralNetwork::from_genome(&genome),
            genome: genome,
        }
    }

    /// Create an AI from a genome
    pub fn with_genome(genome: [f32; 17323]) -> Self {
        AI {
            neuralnet: NeuralNetwork::from_genome(&genome),
            genome: genome,
        }
    }
}

