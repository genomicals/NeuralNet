use rand::Rng;
use pyo3::prelude::*;
use crate::{neural_network::NeuralNetwork, errors::NeuralNetError};

/// AI struct
//#[pyclass]
pub struct AI {
    neuralnet: NeuralNetwork,
    pub genome: [f32; 17323], //weights + biases + mutability
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

/// Generates a new genome from to parents.
pub fn reproduce(parent0: AI, parent1: AI) -> Result<[f32; 17323], NeuralNetError> {
    if  parent0.genome.len() == parent1.genome.len() {
        //first get the cut_index, which will be the point where we cut the genome for the first parent.
        let cut_index: usize = rand::thread_rng().gen_range(0..17323);
        let mut genome: [f32; 17323] = [0.0; 17323];
        let slice0: &[f32];
        let slice1: &[f32];

        if rand::thread_rng().gen_bool(0.5) {
            slice0 = &parent0.genome[0..cut_index];
            slice1 = &parent1.genome[cut_index..];
        } else {
            slice1 = &parent0.genome[0..cut_index];
            slice0 = &parent1.genome[cut_index..];
        }
        genome[0..cut_index].copy_from_slice(slice0); //copy to first part of new genome
        genome[cut_index..].copy_from_slice(slice1); //copy to second part of new genome
        
        /*

        1. Generate random number of mutations (controlled by sig(mutability)*1000 + 5) // upto 5%ish genome mutations // let x be the number of mutations
        2. For each mutation, figure out how intense the mutation is (not controlled by mutability) // max mutation range +-(0.5*5/x)
        3. Enact the mutations

        */ 
        let num_mutations: usize = (genome[17323].tanh() * 500.0 + 505.0).round() as usize; // [5, 1005]
        let mut mutability_range: f32 = 0.0;
        let mut index: usize = 0;
        
        for _ in 0..num_mutations {
            index = rand::thread_rng().gen_range(0..17323);
            mutability_range = 0.75 * (genome[17323] + 0.01);
            genome[index] = (genome[index] + rand::thread_rng().gen_range(-mutability_range..mutability_range)).max(2.0).min(-2.0); //mutate and cap the weight
        }
        genome[17323] = genome[17323].min(0.0).max(1.0); //cap the mutability

        Ok(genome)
    } else {
        Err(NeuralNetError::ReproMismatchLength)
    }
}
