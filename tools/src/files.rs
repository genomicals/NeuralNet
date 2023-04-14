use std::{fs::{self, File}, env, io::{Write, Read}, path::Path};
use std::io::BufReader;
use std::io::prelude::*;

use crate::{generation::Generation, errors::NeuralNetError, ai::AI};


/// Saves the given generation onto the filesystem
pub fn save_generation(generation: &Generation, name: &str) -> Result<(), NeuralNetError> {
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
    println!("size of generation: {}", generation.ais.len());
    for i in 0..generation.ais.len() {
        println!("on iteration: {}", i);
        gen_file.write_all(&generation.ais[i].genome_as_bytes()); //write the genome for all 1000 AIs
    }
    println!("wow");

    Ok(())
}


/// Loads the specified generation from the filesystem
pub fn load_generation(name: &str) -> Generation {
    if Path::new(name).exists() {

        let file = File::open(name).expect("file wasn't found.");
        let reader = BufReader::new(file);
    
        let numbers: Vec<f32> = reader
            .lines()
            .map(|line| line.unwrap().parse::<f32>().unwrap())
            .collect();
        let mut loadedAIs : Vec<AI> = Vec::new();
        for i in 0..1000 {
            loadedAIs.push(AI::with_genome(numbers[(i*17323)..(i*17323)+17323].to_vec()))
        }
        Generation {
            gen_num: 0,
            ais: loadedAIs
        }
    }
    else {
        todo!() // if the file doesn't exist throw error for failing to load from non-existent file.
    }
}


/// Deletes the given generation from the filesystem
pub fn remove_generation(name: &str) {
    todo!()
}


