use std::io;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PassForgeError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid password length: {0}")]
    InvalidLength(String),

    #[error("Invalid word count: {0}")]
    InvalidWordCount(String),

    #[error("Invalid generation amount: {0}")]
    InvalidGenAmount(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Word list error: {0}")]
    WordListError(String),

    #[error("Strength evaluation error: {0}")]
    StrengthEvaluationError(String),

    #[error("Parse error: {0}")]
    ParseError(#[from] ParseIntError),

    #[error("Random number generation error")]
    RandomError,

    #[error("Unknown error occurred")]
    Unknown,
}
