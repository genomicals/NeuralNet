use std::{mem, sync::{Mutex, Arc}};

use crate::{errors::NeuralNetError, neural_network::NeuralNetwork};
use pyo3::prelude::*;
use rand::{Rng, rngs::ThreadRng};


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
        let vec32 = &self.genome;
        let mut vec8 = Vec::new();

        //println!("here before");
        //let vec8 = unsafe {
        //    let ratio = mem::size_of::<f32>() / mem::size_of::<u8>(); //ratio of the two vector sizes
        //    let length = vec32.len() * ratio; //size of the new vector
        //    let capacity = vec32.capacity() * ratio; //capacity of the new vector
        //    let ptr = vec32.as_ptr() as *mut u8; //get a pointer to the old vector
        //    Vec::from_raw_parts(ptr, length, capacity) //construct the new vector of u8's
        //};
        //drop(vec32);

        let bytes_vec: Vec<[u8; 4]> = vec32.iter().map(|fl| fl.to_ne_bytes()).collect();
        for arr in bytes_vec {
            vec8.extend_from_slice(&arr);
        }

        //println!("here after");
        //x

        vec8
    }

    /// Run the board through the AI, returning a set of sorted moves.
    pub fn calculate(&self, board: &[i8; 32]) -> Vec<u8> {
        let converted_board = board.iter().map(|elem| *elem as f32).collect(); //convert the board into a usable input for the neural net
        let unsorted = self.neuralnet.calculate(converted_board); //do the actual calculation here

        // start sorting the result
        let mut vals_with_indices: Vec<(f32, u8)> = Vec::with_capacity(unsorted.len());
        for i in 0..unsorted.len() {
            vals_with_indices.push((unsorted[i], i as u8)); //convert Vec<f32> into Vec<(f32, usize)>
        }
        vals_with_indices.sort_unstable_by(|left, right| left.0.partial_cmp(&right.0).unwrap()); //sort by likeliness
        
        vals_with_indices.iter().map(|x| x.1).collect() //return the actions without the likeliness ratings
    }


    /// Generate a genome from a set of bytes
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
pub fn reproduce(parent0: &AI, parent1: &AI, rng: &mut ThreadRng) -> Result<Vec<f32>, NeuralNetError> {
    //returns size 17323
    if parent0.genome.len() != parent1.genome.len() {
        return Err(NeuralNetError::ReproMismatchLength);
    }

    //first get the cut_index, which will be the point where we cut the genome for the first parent.
    //let c = rand::thread_rng();
    //let cut_index: usize = rand::thread_rng().gen_range(0..17323);
    let cut_index = rng.gen_range(0..17323);
    let mut genome: Vec<f32> = vec![0.0; 17323];
    let slice0: &[f32];
    let slice1: &[f32];

    //if rand::thread_rng().gen_bool(0.5) {
    //    slice0 = &parent0.genome[0..cut_index];
    //    slice1 = &parent1.genome[cut_index..];
    //    genome[0..cut_index].copy_from_slice(slice0); //copy to first part of new genome
    //    genome[cut_index..].copy_from_slice(slice1); //copy to second part of new genome
    //} else {
    //    slice1 = &parent0.genome[0..cut_index];
    //    slice0 = &parent1.genome[cut_index..];
    //    genome[0..cut_index].copy_from_slice(slice1); //copy to first part of new genome
    //    genome[cut_index..].copy_from_slice(slice0); //copy to second part of new genome
    //}

    let n_parent0;
    let n_parent1;
    if rand::thread_rng().gen_bool(0.5) {
        n_parent0 = parent0;
        n_parent1 = parent1;
    } else {
        n_parent0 = parent1;
        n_parent1 = parent0;
    }

    slice0 = &n_parent0.genome[0..cut_index];
    slice1 = &n_parent1.genome[cut_index..];
    genome[0..cut_index].copy_from_slice(slice0); //copy to first part of new genome
    genome[cut_index..].copy_from_slice(slice1); //copy to second part of new genome

    /*

    1. Generate random number of mutations (controlled by sig(mutability)*1000 + 5) // upto 5%ish genome mutations // let x be the number of mutations
    2. For each mutation, figure out how intense the mutation is (not controlled by mutability) // max mutation range +-(0.5*5/x)
    3. Enact the mutations

    */
    let num_mutations: usize = (genome[17322].tanh() * 500.0 + 505.0).round() as usize; // [5, 1005]
    let mut mutability_range: f32;
    let mut index: usize;

    for _ in 0..num_mutations {
        index = rand::thread_rng().gen_range(0..17323);
        mutability_range = 0.75 * (genome[17322].abs() + 0.01);
        genome[index] = (genome[index]
            + rand::thread_rng().gen_range(-mutability_range..mutability_range))
        .max(2.0)
        .min(-2.0); //mutate and cap the weight
    }
    genome[17322] = genome[17322].min(0.0).max(1.0); //cap the mutability

    Ok(genome)
}

pub fn gen_thousand() -> Vec<Arc<Mutex<AI>>> {
    (0..1000).map(|_| Arc::new(Mutex::new(AI::new()))).collect()
}

