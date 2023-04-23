use std::sync::Arc;
use std::sync::Mutex;
use std::{env, fs::{self, File}, io::{Read, Write}, path::Path};

use crate::{ai::AI, errors::NeuralNetError};


/// Saves the given generation onto the filesystem
pub fn save_generation(ais: Arc<Mutex<Vec<Arc<Mutex<AI>>>>>, name: &str) -> Result<(), NeuralNetError> {
    let ais = ais.lock().unwrap();
    let cur = env::current_dir(); //grabs current directory
    if let Err(_) = cur {
        return Err(NeuralNetError::GenerationNotSaved); //handles error
    }
    let binding = cur.unwrap();
    let cur_buf = binding.to_str();
    if let None = cur_buf {
        return Err(NeuralNetError::GenerationNotSaved); //handles error
    }
    let cur = String::from(cur_buf.unwrap()); //the current directory as a string
    let gen = cur + "/generations";
    fs::create_dir(&gen); //create the generations folder if it doesn't exist
    let gen_file_loc = gen + "/" + name + ".gen";
    fs::remove_file(&gen_file_loc);
    let mut gen_file = std::fs::File::create(&gen_file_loc).unwrap(); //retrieve a file struct

    for i in 0..ais.len() {
        //println!("on iteration: {}", i);
        if i == 0 {
            let x = &ais[i].lock().unwrap().genome_as_bytes();
        }
        gen_file.write_all(&ais[i].lock().unwrap().genome_as_bytes()); //write the genome for all 1000 AIs
    }

    drop(gen_file);

    Ok(())
}


/// Loads the specified generation from the filesystem
pub fn load_generation(name: &str) -> Result<Vec<Arc<Mutex<AI>>>, NeuralNetError> {
    let cur = env::current_dir(); //grabs current directory
    if let Err(_) = cur {
        return Err(NeuralNetError::GenerationNotLoaded); //handles error
    }

    let binding = cur.unwrap();
    let cur_buf = binding.to_str();
    if let None = cur_buf {
        return Err(NeuralNetError::GenerationNotLoaded); //handles error
    }

    let cur = String::from(cur_buf.unwrap()); //the current directory as a string

    let gen_file_loc = cur + "/generations" + "/" + name + ".gen"; // The file path to the .gen file

    if !Path::new(&gen_file_loc).exists() {
        return Err(NeuralNetError::GenFileNotFound); //return error if path doesn't exist
    }

    let mut file = File::open(gen_file_loc).map_err(|_| NeuralNetError::GenFileNotFound)?;

    let mut checked = true;
    let mut floats = Vec::new();
    loop {
        use std::io::ErrorKind;
        let mut buffer = [0u8; std::mem::size_of::<f32>()]; //size of an f32
        let res = file.read_exact(&mut buffer); //read from file into res
        
        if checked {
            checked = false;
        }

        match res {
            // we detect if we read until the end.
            // if there were some excess bytes after last read, they are lost.
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => break,
            _ => {}
        }
        res.map_err(|_| NeuralNetError::GenerationNotLoaded)?; //return errors
        let f = f32::from_le_bytes(buffer); //assuming little-endian, may switch to big-endian
        floats.push(f);
    }

    Ok(floats.chunks(17323).map(|chunk| Arc::new(Mutex::new(AI::with_genome(chunk.to_vec())))).collect())
}


/// Deletes the given generation from the filesystem
pub fn remove_generation(name: &str) {
    todo!()
}
