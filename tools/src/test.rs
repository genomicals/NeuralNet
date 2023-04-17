use crate::{
    ai::AI,
    errors, files,
    generation::{self, Generation},
};
use std::{env, mem::MaybeUninit};

struct TestStruct {
    x: i32,
}
impl TestStruct {
    pub fn new() -> Self {
        TestStruct { x: 2 }
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

    println!("here before loading");

    let l_gen = files::load_generation("test_gen"); // Load in the generation.
    if let Err(e) = l_gen {
        // received an error instead.
        println!("{}", e);
        println!("Received an error from load_generation.");
        return false;
    }

    println!("innit bruv");

    let l_gen = l_gen.unwrap();
    // check if each of the ai's match.. just in case.
    //gen.ais.iter().reduce(|acc, e| )
    for i in 0..gen.ais.len() {
        if gen.ais[i] != l_gen.ais[i] {
            println!("Ai #:{} failed", i);
            println!("{} vs {}", gen.ais[i].genome[0], l_gen.ais[i].genome[0]);
            return false;
        }
    }
    println!("bruh");
    return true;
}
