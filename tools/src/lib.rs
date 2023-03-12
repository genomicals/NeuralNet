mod errors;
mod neural_network;
mod ai;
mod files;
mod generation;
mod test;

use pyo3::prelude::*;
use ai::AI;


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


/// A Python module implemented in Rust.
#[pymodule]
fn tools(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    //m.add_class::<AI>()?;
    Ok(())
}

