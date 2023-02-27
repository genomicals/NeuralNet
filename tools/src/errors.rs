use core::fmt;

/// Custom error type for the NeuralNet library
#[derive(Debug)]
pub enum NeuralNetError {

}
impl std::error::Error for NeuralNetError {}
impl fmt::Display for NeuralNetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Errors to be implemented")
    }
}




