//! This module defines custom error types for the PassForge library.
//!
//! It provides a comprehensive set of errors that can occur during password
//! and passphrase generation, configuration, and strength evaluation.

use std::io;
use std::num::ParseIntError;
use thiserror::Error;

/// Represents all possible errors that can occur in the PassForge library.
#[derive(Error, Debug)]
pub enum PassForgeError {
    /// Represents errors that occur during I/O operations.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Represents errors related to invalid password length.
    #[error("Invalid password length: {0}")]
    InvalidLength(String),

    /// Represents errors related to invalid word count for passphrases.
    #[error("Invalid word count: {0}")]
    InvalidWordCount(String),

    /// Represents errors related to invalid generation amount.
    #[error("Invalid generation amount: {0}")]
    InvalidGenAmount(String),

    /// Represents errors related to invalid configuration.
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Represents errors related to word list issues.
    #[error("Word list error: {0}")]
    WordListError(String),

    /// Represents errors that occur during strength evaluation.
    #[error("Strength evaluation error: {0}")]
    StrengthEvaluationError(String),

    /// Represents errors that occur during parsing of numeric values.
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseIntError),

    /// Represents errors related to random number generation.
    #[error("Random number generation error")]
    RandomError,

    /// Represents unknown or unexpected errors.
    #[error("Unknown error occurred")]
    Unknown,
}