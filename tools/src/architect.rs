use rand::{rngs::ThreadRng, Rng};

use crate::{engine::{self, Engine, Action}, generation::Generation, AI};
use std::convert;
use rand::seq::SliceRandom;
use rand::thread_rng;

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
        // shuffle the bracket 
        let mut rng = thread_rng();
        self.bracket.shuffle(&mut rng);
    }

    // Runs all the games in bracket order.
    pub fn run_games(&mut self) {
        // each loop creates a round robin style tournament of 1v1v1v1 (4 players)
        for i in 0..250 {
            let i = i*4;
            let mut result;
            let p0 = &self.generation.ais[self.bracket[i]];
            let p1 = &self.generation.ais[self.bracket[i+1]];
            let p2 = &self.generation.ais[self.bracket[i+2]];
            let p3 = &self.generation.ais[self.bracket[i+3]];

            // run all combinations of games for our four ais
            result = Architect::run_game(&p0, &p1, &mut self.rng);
            self.fitness[self.bracket[i]] += result.0;
            self.fitness[self.bracket[i+1]] += result.1;

            result = Architect::run_game(&p0, &p2, &mut self.rng);
            self.fitness[self.bracket[i]] += result.0;
            self.fitness[self.bracket[i+2]] += result.1;

            result = Architect::run_game(&p0, &p3, &mut self.rng);
            self.fitness[self.bracket[i]] += result.0;
            self.fitness[self.bracket[i+3]] += result.1;

            result = Architect::run_game(&p1, &p2, &mut self.rng);
            self.fitness[self.bracket[i+1]] += result.0;
            self.fitness[self.bracket[i+2]] += result.1;

            result = Architect::run_game(&p1, &p3, &mut self.rng);
            self.fitness[self.bracket[i+1]] += result.0;
            self.fitness[self.bracket[i+3]] += result.1;

            result = Architect::run_game(&p2, &p3, &mut self.rng);
            self.fitness[self.bracket[i+2]] += result.0;
            self.fitness[self.bracket[i+3]] += result.1;
        }
    }

    // Runs a single game between to AI players and returns their fitness score.
    pub fn run_game(player1: &AI, player2: &AI, rng: &mut ThreadRng) -> (i32, i32) {
        let player_decider: bool = rng.gen(); //decide if player1 is red or black
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

        // main game loop
        loop {
            let moves = p1.calculate(game.peak_red()); //output of the neural network
            for cur in moves {
                let (tile, action) = Architect::index_to_move(cur);

            }
        }

        return (0, 0);
    }

    /// asdf
    #[inline]
    pub fn index_to_move(cur: usize) -> (u8, Action) {

    }

    /// TODO COMMENT THIS LATER
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
