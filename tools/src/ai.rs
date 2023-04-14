use std::mem;

use crate::{errors::NeuralNetError, neural_network::NeuralNetwork};
use pyo3::prelude::*;
use rand::Rng;


/// AI struct
//#[pyclass]
pub struct AI {
    neuralnet: NeuralNetwork,
    pub genome: Vec<f32>, //size 17323, weights + biases + mutability
                          //#[pyo3(get, set)]
                          //test_int: usize,
}
impl PartialEq for AI {
    fn eq(&self, other: &Self) -> bool {
        self.genome == other.genome
    }
}
//#[pymethods]
impl AI {
    //#[new]
    /// Create an AI with a randomized genome
    pub fn new() -> Self {
        let genome: Vec<f32> = (0..17323)
            .map(|_| rand::thread_rng().gen_range(-1.0..=1.0))
            .collect();
        AI {
            neuralnet: NeuralNetwork::from_genome(&genome),
            genome: genome,
        }
    }

    /// Create an AI from a genome
    pub fn with_genome(genome: Vec<f32>) -> Self {
        AI {
            neuralnet: NeuralNetwork::from_genome(&genome),
            genome: genome,
        }
    }

    /// Return the AI's genome as bytes
    pub fn genome_as_bytes(&self) -> Vec<u8> {
        let mut vec32 = &self.genome;

        println!("here before");
        let vec8 = unsafe {
            let ratio = mem::size_of::<f32>() / mem::size_of::<u8>(); //ratio of the two vector sizes
            let length = vec32.len() * ratio; //size of the new vector
            let capacity = vec32.capacity() * ratio; //capacity of the new vector
            let ptr = vec32.as_ptr() as *mut u8; //get a pointer to the old vector
            Vec::from_raw_parts(ptr, length, capacity) //construct the new vector of u8's
        };
        println!("here after");

        vec8
    }


    pub fn genome_from_bytes(bytes: &[u8]) -> Vec<f32> {


        bytes.chunks(4).map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap())).collect()




        //loop {
        //    use std::io::ErrorKind;
        //    let mut buffer = [0u8; std::mem::size_of::<f32>()]; //size of an f32
        //    let res = bytes.read_exact(&mut buffer); //read from file into res
        //    match res {
        //        // we detect if we read until the end.
        //        // if there were some excess bytes after last read, they are lost.
        //        Err(error) if error.kind() == ErrorKind::UnexpectedEof => break,
        //        _ => {}
        //    }
        //    res.map_err(|_| NeuralNetError::GenerationNotLoaded)?; //return errors
        //    let f = f32::from_le_bytes(buffer); //assuming little-endian, may switch to big-endian
        //    floats.push(f);
        //}
    }
}


/// Generates a new genome from to parents.
pub fn reproduce(parent0: AI, parent1: AI) -> Result<Vec<f32>, NeuralNetError> {
    //returns size 17323
    if parent0.genome.len() != parent1.genome.len() {
        return Err(NeuralNetError::ReproMismatchLength);
    }

    //first get the cut_index, which will be the point where we cut the genome for the first parent.
    let cut_index: usize = rand::thread_rng().gen_range(0..17323);
    let mut genome: Vec<f32> = vec![0.0; 17323];
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
        genome[index] = (genome[index]
            + rand::thread_rng().gen_range(-mutability_range..mutability_range))
        .max(2.0)
        .min(-2.0); //mutate and cap the weight
    }
    genome[17323] = genome[17323].min(0.0).max(1.0); //cap the mutability

    Ok(genome)
}

