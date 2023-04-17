use rand::{rngs::ThreadRng, Rng};

use crate::{engine::{self, Engine}, generation::Generation, AI};
use std::convert;

pub struct Architect {
    pub generation: Generation,
    pub fitness: Vec<i32>,
    //pub bracket: [[usize;4];250], //[250] groups of 4 [0,1,2,3] 0-1,0-2,0-3,1-2,1-3,2-3
    pub bracket: Vec<usize>,
    pub rng: ThreadRng, //using the same engine for rng should increase performance slightly
}

impl Architect {
    // Creates a new Architect.
    pub fn new() -> Self {
        Architect {
            generation: Generation {
                gen_num: 0,
                ais: (0..1000).map(|_| AI::new()).collect(),
            },
            fitness: vec![0; 1000],
            bracket: vec![0; 1000],
            rng: rand::thread_rng(),
        }
    }

    // Creates the tournament bracket, for now this is simple assignment.
    pub fn construct_tournament(&mut self) {
        for i in 0..250 {
            for j in 0..4 {
                self.bracket[i * 4 + j] = j * i;
            }
        }
        // we can assign a different function if we want true random matches.
    }

    // Runs all the games in bracket order.
    pub fn run_games(&mut self) {
        for i in 0..250 {}
    }

    // Runs a single game between to AI players and returns their fitness score. Consists of 3 rounds.
    pub fn run_game(&mut self, player1: &AI, player2: &AI, board: &mut engine::Engine) -> (i32, i32) {
        let player_decider: bool = self.rng.gen(); //decide if player1 is red or black
        let game = Engine::new();
        let p1: &AI;
        let p2: &AI;
        if player_decider {
            p1 = player1;
            p2 = player2;
        } else {
            p1 = player2;
            p2 = player1;
        }

        loop {
            let moves = p1.calculate(game.peak_red());
        }

        return (0, 0);
    }

    pub fn determine_survival(&mut self) {
        let mut fit_clone = self.fitness.clone(); //TODO see if we need to clone this at all, reduce memory
        fit_clone.sort();
        let fitness_metric = fit_clone[fit_clone.len() / 2];
        let mut ofs: i32 = 0;
        for i in 0..1000 {
            if self.fitness[i] < fitness_metric {
                ofs = ofs - 1;
            } else {
                ofs = ofs + 1;
            }
        }
        if ofs == 0 {
            // The math worked out nicely and we can move on to elimination and repro normally.
        } else if ofs < 0 { //Too many marked for elimination.
        } else if ofs > 0 { // Too few marked for elimination.
        }
    }
}
