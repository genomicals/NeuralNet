use pyo3::prelude::*;
use crate::neural_network::NeuralNetwork;

/// AI struct
#[pyclass]
pub struct AI {
    neuralnet: NeuralNetwork,
    #[pyo3(get, set)]
    test_int: usize,
}
#[pymethods]
impl AI {
    #[new]
    pub fn new() -> Self {
        AI {
            neuralnet: NeuralNetwork::new(),
            test_int: 3,
        }
    }
}

