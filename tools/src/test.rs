use crate::{files, generation::{self, Generation}, ai::AI, errors};
use std::{mem::MaybeUninit, env};

struct TestStruct {
    x: i32,
}
impl TestStruct {
    pub fn new() -> Self {
        TestStruct {x: 2}
    }
}


pub fn thing_test() -> bool {
    println!("waddup");
    //let y = TestStruct {x: 3};
    //let x: [f32; 20000] = [(); 20000].map(|_| 0.0);
    //let a = AI::new();
    //let ar: [TestStruct; 1000] = [(); 1000].map(|_| TestStruct::new()); //this is fine
    //let aar: [AI; 1] = [(); 1].map(|_| AI::new()); //this is fine

    //let mut aar: [AI; 4] = unsafe {MaybeUninit::uninit().assume_init()};
    //for i in 0..4 {
    //    aar[i] = AI::new();
    //}

    //let mut aar: [AI; 1000] = unsafe {MaybeUninit::uninit().assume_init()};
    //for i in 0..1000 {
    //    aar[i] = AI::new();
    //}

    //let aaar: [AI; 2] = [(); 2].map(|_| AI::new()); //this is not fine
    //let aar: [AI; 3] = [(); 3].map(|_| AI::new()); //this is not fine
    //let aar: [AI; 5] = [(); 5].map(|_| AI::new()); //this is not fine
    //let arr: [AI; 10] = [(); 10].map(|_| AI::new());
    //let gen = Generation::new();

    return true;
}

/// Test the files modules
pub fn test_files() -> bool {

    let gen = Generation::new(); // Create new Generation
    
    let res = files::save_generation(&gen, "test_gen"); // Save the Generation
    if let Err(_) = res {
        println!("In the Err");
        return false;
    }

    let cur = env::current_dir(); //grabs current directory
    if let Err(_) = cur {
        //return Err(NeuralNetError::GenerationNotSaved); //handles error
        return  false;
    }

    let binding = cur.unwrap();
    let cur_buf = binding.to_str();
    if let None = cur_buf {
        //return Err(NeuralNetError::GenerationNotSaved); //handles error
        return  false;
    }

    let cur = String::from(cur_buf.unwrap()); //the current directory as a string

    let gen_file_loc = cur + "/generations" + "/" + "test_gen" + ".gen"; // The file path to the .gen file

    let l_gen = files::load_generation(&gen_file_loc); // Load in the generation.

    // check if each of the ai's match.. just in case.
    for i in 0..gen.ais.len() {
        if gen.ais[i] != l_gen.ais[i] {
            return false
        }
    }
    return true;
}

