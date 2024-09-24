//! This module defines the interface for password strength evaluation.
//!
//! It provides a `StrengthEvaluator` trait that can be implemented by different
//! strength evaluation algorithms, allowing for flexible and extensible password
//! strength checking.

use std::fmt::Display;

use crate::PassForgeError;

/// The `StrengthEvaluator` trait defines the interface for password strength evaluation.
///
/// This trait allows for a common interface across different types of strength evaluators,
/// enabling easy swapping and extension of evaluation algorithms.
pub trait StrengthEvaluator {
    /// The input type for the strength evaluator.
    type Input;
    /// The output type produced by the strength evaluator, which must implement `Display`.
    type Output: Display;

    /// Checks if the input passes a predefined strength threshold.
    ///
    /// # Arguments
    ///
    /// * `input` - A reference to the input to evaluate.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a boolean indicating whether the input passes the threshold,
    /// or a `PassForgeError` if an error occurred during evaluation.
    fn passes_threshold(input: &Self::Input) -> Result<bool, PassForgeError>;

    /// Evaluates the strength of the input.
    ///
    /// # Arguments
    ///
    /// * `input` - A reference to the input to evaluate.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the strength evaluation output if successful,
    /// or a `PassForgeError` if an error occurred during evaluation.
    fn evaluate(input: &Self::Input) -> Result<Self::Output, PassForgeError>;
}

pub mod zxcvbn_analysis;

pub use zxcvbn_analysis::ZxcvbnAnalysis;
