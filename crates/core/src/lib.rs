pub mod types;

pub use types::{CipherType, ClassicalCipher, DetectionResult, EncodingType, HashType};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CipherError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Detection failed: {0}")]
    DetectionFailed(String),
    
    #[error("Analysis error: {0}")]
    AnalysisError(String),
}

pub type Result<T> = std::result::Result<T, CipherError>;
