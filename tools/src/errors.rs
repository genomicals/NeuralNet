use core::fmt;

/// Custom error type for the NeuralNet library
#[derive(Debug)]
pub enum NeuralNetError {
    InvalidInputSize,
    InvalidWeightSize,
    ReproMismatchLength,
}
impl std::error::Error for NeuralNetError {}
impl fmt::Display for NeuralNetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInputSize => write!(f, "Input was not size 32"),
            Self::InvalidWeightSize => write!(f, "Input was not size _"),
            Self::ReproMismatchLength => write!(f, "The two parents had mismatched length genomens."),
        }
    }
}

