use std::fmt;

/// Main error type for the application
#[derive(Debug, Clone)]
pub enum Error {
    /// Invalid grid dimensions
    InvalidDimensions { width: usize, height: usize },
    
    /// Position out of bounds
    OutOfBounds { x: usize, y: usize, width: usize, height: usize },
    
    /// Invalid rule string format
    InvalidRuleFormat(String),
    
    /// Pattern parsing error
    PatternParseError(String),
    
    /// I/O error
    IoError(String),
    
    /// Generic error with message
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidDimensions { width, height } => {
                write!(f, "Invalid grid dimensions: {}x{}", width, height)
            }
            Error::OutOfBounds { x, y, width, height } => {
                write!(
                    f,
                    "Position ({}, {}) is out of bounds for grid {}x{}",
                    x, y, width, height
                )
            }
            Error::InvalidRuleFormat(msg) => write!(f, "Invalid rule format: {}", msg),
            Error::PatternParseError(msg) => write!(f, "Pattern parse error: {}", msg),
            Error::IoError(msg) => write!(f, "I/O error: {}", msg),
            Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type alias for the application
pub type Result<T> = std::result::Result<T, Error>;