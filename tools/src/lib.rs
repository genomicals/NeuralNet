mod ai;
mod architect;
mod engine;
mod errors;
mod files;
mod neural_network;
mod test;

use std::sync::{Mutex, Arc};

use ai::AI;
use architect::Architect;
use engine::{Engine, CheckersResult};
use pyo3::prelude::*;


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


/// Runs internal module tests that cannot be tested by Python
#[pyfunction]
fn run_tests() -> bool {
    println!("waddup before test_files");

    //if !test::test_files() {
    //    return false;
    //}
    if !test::test_game_manual() {
        return false;
    }
    if !test::test_game() {
        return false;
    }
    if !test::test_reproduce() {
        return false;
    }
    if !test::test_evolve() {
        return false;
    }
    println!("waddup end");

    true
}


/// The main way to train and interact with the AI
#[pyclass]
struct GenerationManager {
    architect: Architect
}
#[pymethods]
impl GenerationManager {
    #[new]
    fn new() -> Self {
        GenerationManager {architect: Architect::new()}
    }

    /// Trains the AI for a certain number of generations
    fn train_generations(&mut self, num_generations: usize) {
        for _ in 0..num_generations {
            self.architect.evolve_generation()
        }
    }

    /// Trains the AI for a single generation
    fn train_generation(&mut self) {
        self.architect.evolve_generation();
    }

    /// Starts a game with the current best AI
    fn create_game(&self) -> GameManager {
        GameManager::with_ai(self.architect.generation.lock().unwrap()[0].clone())
    }

    /// Save the current generation under the given name
    fn save_generation(&self, name: &str) {
        files::save_generation(self.architect.generation.clone(), name).unwrap(); //panic because why not
    }

    /// Load the generation with the given name
    fn load_generation(&mut self, name: &str) {
        let result = files::load_generation(name);
        if let Ok(v) = result {
            self.architect.generation = Arc::new(Mutex::new(v));
        }
    }
}


/// Used to play against a particular AI
#[pyclass]
struct GameManager {
    ai: Arc<Mutex<AI>>,
    game: Engine,
    player: bool,
}
impl GameManager {
    pub fn with_ai(ai: Arc<Mutex<AI>>) -> Self {
        GameManager {ai: ai, game: Engine::new(), player: false}
    }
}
#[pymethods]
impl GameManager {
    /// Initialize a new game
    fn new_game(&mut self, player: bool) {
        self.player = player;
        self.game = Engine::new();

        if !player { //player is black, so they make the first move
            return;
        }

        //player is red, so the ai makes the first move
        let moves = self.ai.lock().unwrap().calculate(self.game.peek_black());

        // find the first valid move
        for cur in moves {
            let (tile, action) = Architect::index_to_move(cur);
            let result = self.game.make_move(tile, action);
            if let Ok(_) = result {
                return; //once a good move has happened, let the player make their first move
            }
        }
    }

    /// Allows the Player to take their turn, and then has the AI take its turn. NOTE: no way of knowing if the player gets stalemated
    pub fn make_move(&mut self, tile: u8, action: u8) -> i32 {
        let result = self.game.make_move(tile, Architect::num_to_action(action));
        match result {
            Err(_) => return 0, //invalid move from player
            Ok(CheckersResult::Win(player)) => if player == self.player {return 1} else {return 2},
            Ok(CheckersResult::Ok(player)) => {
                if player == self.player {return 0;} //if it's the player's move, make them go again

                'ai_turn: loop {
                    let moves = if self.player { //if player is red, ai is black
                        self.ai.lock().unwrap().calculate(&self.game.board_black) //get move from ai
                    } else {
                        self.ai.lock().unwrap().calculate(&self.game.board_red)
                    };

                    // find the first valid move
                    for cur in moves {
                        let (tile, action) = Architect::index_to_move(cur);
                        let result = self.game.make_move(tile, action);
                        match result {
                            Err(_) => continue,
                            Ok(CheckersResult::Win(player)) => if player == self.player {return 1} else {return 2},
                            Ok(CheckersResult::Ok(player)) => if player == self.player {return 0} else {continue 'ai_turn}, //hand the turn to the player or the ai
                        }
                    }

                    // if we reach here, the ai has exhausted all its possible moves
                    return 1;
                }
            },
        }
    }

    /// Returns the current board as a list TODO need to figure out how to create and return python lists
    fn peek_board(&self) -> [i8; 32] {
        if self.player {self.game.peek_red_python()} else {self.game.peek_black_python()}
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn tools(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(run_tests, m)?)?;
    m.add_class::<GenerationManager>()?;
    m.add_class::<GameManager>()?;
    Ok(())
}



