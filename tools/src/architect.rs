use std::convert;
use crate::{AI, engine, generation::Generation};

pub struct Architect {
    pub generation: Generation,
    pub fitness: [i32; 1000],
    pub bracket: [[usize;4];250],
}

impl Architect {
    // Creates a new Architect.
    pub fn new() -> Self {
        Architect { 
            generation: Generation{gen_num: 0, ais: [();1000].map(|_| AI::new())},
            fitness: [0; 1000],
            bracket: [[0;4];250],
        }    
    }

    // Creates the tournament bracket, for now this is simple assignmeft.
    pub fn construct_tournament(&mut self) {
        for i in 0..250 {
            for j in 0..4 {
                self.bracket[i][j] = j*i; 
            }
        }
        // we can assign a different function if we want true random matches.
    }

    // Runs all the games in bracket order.
    pub fn run_games(&mut self) {
        for i in 0..250 {
            
        }
    }

    // Runs a single game between to AI players and returns their fitness score.
    pub fn run_game(player1: AI, player2: AI , board : engine::Engine) -> (i32, i32) {
        //
        return (0,0);
    }

    pub fn determine_survival(&mut self) {
        let mut fit_clone = self.fitness.clone();
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
        }
        else if ofs < 0 { //Too many marked for elimination.
            
        }
        else if ofs > 0 { // Too few marked for elimination.

        }
    }
}

