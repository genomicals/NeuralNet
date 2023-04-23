use crate::{
    ai::AI,
    errors, files,
    generation::{self, Generation}, architect::Architect, engine::{Engine, Action},
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
    let scores = Architect::run_game(&p1, &p2, &mut rng);
    println!("scores: {:?}", scores);
    return true;
}

/// Test the files modules
pub fn test_files() -> bool {
    println!("here generating a new generation");
    let gen = Generation::new(); // Create new Generation

    println!("here before saving");
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
