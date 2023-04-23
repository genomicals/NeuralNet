use crate::{
    ai::{AI, self},
    errors, files,
    architect::Architect, engine::{Engine, Action},
};
use std::{env, mem::MaybeUninit, ops::Deref, sync::{Arc, Mutex}};

struct TestStruct {
    x: i32,
}
impl TestStruct {
    pub fn new() -> Self {
        TestStruct { x: 2 }
    }
}

pub fn test_game_manual() -> bool {
    let p1 = AI::new();
    let p2 = AI::new();
    let mut rng = rand::thread_rng();
    let mut game = Engine::new();
    //let scores = Architect::run_game(&p1, &p2, &mut rng);
    //println!("scores: {:?}", scores);
    println!("{:?}", game.peek_black());
    let result = game.make_move(17, Action::MoveNorthwest);
    println!("results are in: {:?}", result);
    println!("{:?}", game.peek_black());
    let result = game.make_move(21, Action::MoveNortheast);
    println!("results are in: {:?}", result);
    println!("{:?}", game.peek_black());
    println!("current piece counts: {}, {}", game.black_pieces, game.red_pieces);
    let result = game.make_move(28, Action::MoveNortheast);
    println!("results are in: {:?}", result);
    let result = game.make_move(30, Action::MoveNorthwest);
    println!("results are in: {:?}", result);
    let result = game.make_move(28, Action::JumpNortheast);
    println!("results are in: {:?}", result);
    return true;
}

pub fn test_game() -> bool {
    let p1 = AI::new();
    let p2 = AI::new();
    let mut rng = rand::thread_rng();
    let mut game = Engine::new();
    let scores = Architect::run_game(Arc::new(Mutex::new(p1)), Arc::new(Mutex::new(p2)), &mut rng);
    println!("scores: {:?}", scores);
    return true;
}


pub fn test_reproduce() -> bool {
    //let p1 = Arc::new(Mutex::new(AI::new()));
    //let p2 = Arc::new(Mutex::new(AI::new()));
    let p1 = AI::new();
    let p2 = AI::new();
    let mut rng = rand::thread_rng();
    let x = ai::reproduce(&p1, &p2, &mut rng);

    true
}

pub fn test_evolve() -> bool {
    let mut arch = Architect::new();
    arch.evolve_generation();

    return true;
}

/// Test the files modules
pub fn test_files() -> bool {
    println!("here generating a new generation");
    //let gen = Generation::new(); // Create new Generation
    let ais = Arc::new(Mutex::new(ai::gen_thousand()));

    println!("here before saving");
    let res = files::save_generation(ais.clone(), "test_gen"); // Save the Generation
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
    let ais = ais.lock().unwrap();
    for i in 0..ais.len() {
        if ais[i].lock().unwrap().deref() != l_gen[i].lock().unwrap().deref() {
            println!("Ai #:{} failed", i);
            println!("{} vs {}", ais[i].lock().unwrap().genome[0], l_gen[i].lock().unwrap().genome[0]);
            return false;
        }
    }
    println!("bruh");
    return true;
}
