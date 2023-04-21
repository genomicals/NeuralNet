mod ai;
mod architect;
pub mod engine;
mod errors;
mod files;
mod generation;
mod neural_network;
mod test;

use ai::AI;
use architect::Architect;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::py_run;


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


/// Runs internal module tests that cannot be tested by Python
#[pyfunction]
fn run_tests() -> bool {
    let result: bool;

    if !test::thing_test() {
        return false;
    }
    println!("waddup before test_files");

    if !test::test_files() {
        return false;
    }
    println!("waddup end");

    true
}


/// The main way to train and interact with the AI
#[pyclass]
struct GenerationManager {
    #[pyo3(get, set)]
    test_num: f32,

}
#[pymethods]
impl GenerationManager {
    #[new]
    fn new() -> Self {
        GenerationManager {test_num: 56.0}
    }

    /// Trains the AI for a certain number of generations
    fn train_generations(self_: PyRef<'_, Self>, num_generations: usize) {
        todo!();
    }

    /// Starts a game with the current best AI
    fn start_game(self_: PyRef<'_, Self>) -> GameManager {
        todo!();
    }
}


/// Used to play against a particular AI
#[pyclass]
struct GameManager {
    #[pyo3(get, set)]
    test_num1: f32,
}
#[pymethods]
impl GameManager {
    #[new]
    fn new() -> Self {
        GameManager {test_num1: 0.0}
    }

    /// Initialize a new game
    fn new_game(self_: PyRef<'_, Self>) {

    }

    /// Makes the current player's move
    pub fn make_move(self_: PyRef<'_, Self>, tile: u32, action: u32) -> i32 {
        // mut self_: PyRefMut<'_, Self>
        //todo!();
        return 5;
    }

    /// Returns the current board as a list TODO need to figure out how to create and return python lists
    fn peak_board(self_: PyRef<'_, Self>) -> [i8; 32] {
        //todo!();
        return [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
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



