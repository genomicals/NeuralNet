use std::{fs, env};
use crate::{generation::Generation, errors::NeuralNetError};


/// Saves the given generation onto the filesystem
pub fn save_generation(generation: &Generation, name: &str) -> Result<(), NeuralNetError> {
    let cur = env::current_dir();
    if let Err(_) = cur {
        return Err(NeuralNetError::GenerationNotSaved);
    }
    let binding = cur.unwrap();
    let cur_buf = binding.to_str();
    if let None = cur_buf {
        return Err(NeuralNetError::GenerationNotSaved);
    }
    let cur = cur_buf.unwrap();

    Ok(())
}


/// Loads the specified generation from the filesystem
pub fn load_generation(name: &str) -> Generation {
    todo!()
}


/// Deletes the given generation from the filesystem
pub fn remove_generation(name: &str) {
    todo!()
}


