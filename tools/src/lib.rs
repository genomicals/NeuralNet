use core::fmt;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::py_run;


/*
 * Can take in a &PyDict
 *
 *
 * */


/// Neural network struct
#[pyclass]
struct NeuralNetwork {
    internal: f32,
    #[pyo3(get, set)]
    external: f32,
}
#[pymethods]
impl NeuralNetwork {
    #[new]
    fn new(external: f32) -> Self {
        NeuralNetwork {internal: 0.0, external: external}
    }

    fn update_internal(&mut self, internal: f32) {
        self.internal = internal;
    }

    fn calculate(&self) -> f32 {
        self.internal * self.external
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    //if x < 0 {Err(PyValueError::new_error("x is negative"))}
    Ok((a + b).to_string())
}


/// A Python module implemented in Rust.
#[pymodule]
fn tools(_py: Python, m: &PyModule) -> PyResult<()> {
    // Create tools module
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    //m.add_function(wrap_pyfunction!(test_mod::test_func, m)?)?;
    m.add_class::<NeuralNetwork>()?;

    // Create tools.test_mod
    //let sm = PyModule::new(_py, "tools.test_mod")?;
    //sm.add_function(wrap_pyfunction!(test_mod::test_func, sm)?)?;
    //py_run!(_py, sm, "import sys; sys.modules['tools.test_mod'] = test_mod");
    //m.add_submodule(sm)?;

    Ok(())
}


/// Custom error type for the NeuralNet library
#[derive(Debug)]
struct NeuralNetError;
impl std::error::Error for NeuralNetError {}
impl fmt::Display for NeuralNetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Oh no!")
    }
}

