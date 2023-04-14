use core::fmt;

/// Custom error type for the NeuralNet library
#[derive(Debug)]
pub enum NeuralNetError {
    InvalidInputSize,
    InvalidWeightSize,
    ReproMismatchLength,
    GenerationNotSaved,
    GenerationNotLoaded,
    GenFileNotFound,
}
impl std::error::Error for NeuralNetError {}
impl fmt::Display for NeuralNetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInputSize => write!(f, "Input was not size 32."),
            Self::InvalidWeightSize => write!(f, "Input was not size _."),
            Self::ReproMismatchLength => write!(f, "The two parents had mismatched length genomes."),
            Self::GenerationNotSaved => write!(f, "Couldn't save the generation."),
            Self::GenerationNotLoaded => write!(f, "Couldn't load the generation."),
            Self::GenFileNotFound => write!(f, ".gen file not found; couldn't load the generation."),
        }
    }
}


/// Custom error type for the checkers engine
#[derive(Debug)]
pub enum CheckersError {
    IllegalMove,
}
impl std::error::Error for CheckersError {}
impl fmt::Display for CheckersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IllegalMove => write!(f, "This move was invalid."),
        }
    }
}
