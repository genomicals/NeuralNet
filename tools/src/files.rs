use std::io::prelude::*;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::Mutex;
use std::{env, fs::{self, File}, io::{Read, Write}, path::Path};

use crate::{ai::AI, errors::NeuralNetError};


/// Saves the given generation onto the filesystem
pub fn save_generation(ais: &Vec<Arc<Mutex<AI>>>, name: &str) -> Result<(), NeuralNetError> {
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

    println!("here");
    println!("size of generation: {}", ais.len());
    for i in 0..ais.len() {
        //println!("on iteration: {}", i);
        if i == 0 {
            let x = &ais[i].lock().unwrap().genome_as_bytes();
            println!("0th: {}", x[0]);
        }
        gen_file.write_all(&ais[i].lock().unwrap().genome_as_bytes()); //write the genome for all 1000 AIs
    }
    println!("wow");

    drop(gen_file);

    println!("wow right before");

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
    //let reader = BufReader::new(file);

    //let numbers: Vec<f32> = reader
    //    .lines()
    //    .map(|line| line.unwrap().parse::<f32>().unwrap())
    //    .collect();
    //let numbers = f32::from_be_bytes();
    println!("here");


    let mut checked = true;
    let mut floats = Vec::new();
    loop {
        use std::io::ErrorKind;
        let mut buffer = [0u8; std::mem::size_of::<f32>()]; //size of an f32
        let res = file.read_exact(&mut buffer); //read from file into res
        
        if checked {
            checked = false;
            println!("0th read: {}", buffer[0]);
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



    // TODO, DO THIS STUFF LATER
    //let mut buf = vec![0; 17323];
    //file.read(&mut buf);

    //let genome = AI::genome_from_bytes(&buf);
    



    //Ok(Generation {
    //    gen_num: 0,
    //    ais: floats.chunks(17323).map(|chunk| AI::with_genome(chunk.to_vec())).collect(),
    //})
    Ok(floats.chunks(17323).map(|chunk| Arc::new(Mutex::new(AI::with_genome(chunk.to_vec())))).collect())
}


/// Deletes the given generation from the filesystem
pub fn remove_generation(name: &str) {
    todo!()
}
