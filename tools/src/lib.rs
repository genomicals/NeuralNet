mod errors;
mod neural_network;
mod ai;
mod architect;
mod files;
mod generation;
mod test;
pub mod engine;

use pyo3::prelude::*;
use architect::Architect;
use ai::AI;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}



/// Runs internal module tests that cannot be tested by Python
#[pyfunction]
fn run_tests() -> PyResult<bool> {
    let result: bool;

    if !test::thing_test() {
        return Ok(false);
    }

    return Ok(false);

    if !test::test_files() {
        return Ok(false);
    }

    Ok(true)
}


/// A Python module implemented in Rust.
#[pymodule]
fn tools(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(run_tests, m)?)?;
    //m.add_class::<AI>()?;
    Ok(())
}

