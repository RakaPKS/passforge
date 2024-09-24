//! This module implements password strength evaluation using the zxcvbn algorithm.
//!
//! It provides a `ZxcvbnAnalysis` struct that implements the `StrengthEvaluator` trait,
//! allowing for detailed password strength analysis.

use zxcvbn::zxcvbn;

use crate::{strength_evaluator::StrengthEvaluator, PassForgeError};

/// Struct for evaluating password strength using the zxcvbn algorithm.
pub struct ZxcvbnAnalysis;

impl ZxcvbnAnalysis {
    /// The minimum score considered as a "pass" for password strength.
    const MIN_PASS_SCORE: u8 = 3;
}

impl StrengthEvaluator for ZxcvbnAnalysis {
    type Input = String;
    type Output = String;

    /// Checks if the password passes the minimum strength threshold.
    ///
    /// # Arguments
    ///
    /// * `input` - A reference to the password to evaluate.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a boolean indicating whether the password passes the threshold,
    /// or a `PassForgeError` if an error occurred during evaluation.
    ///
    /// # Errors
    ///
    /// Will return an error if the input password is empty.
    fn passes_threshold(input: &Self::Input) -> Result<bool, PassForgeError> {
        if input.is_empty() {
            return Err(PassForgeError::InvalidLength(
                "Password cannot be empty".into(),
            ));
        }
        Ok((zxcvbn(input, &[]).score() as u8) >= Self::MIN_PASS_SCORE)
    }

    /// Evaluates the strength of the password using the zxcvbn algorithm.
    ///
    /// # Arguments
    ///
    /// * `input` - A reference to the password to evaluate.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a string with the password strength evaluation,
    /// or a `PassForgeError` if an error occurred during evaluation.
    ///
    /// # Errors
    ///
    /// Will return an error if the input password is empty.
    fn evaluate(input: &Self::Input) -> Result<Self::Output, PassForgeError> {
        if input.is_empty() {
            return Err(PassForgeError::InvalidLength(
                "Input password cannot be empty".into(),
            ));
        }
        let estimate = zxcvbn(input, &[]);
        Ok(format!(
            "Score: {}/4, Crack time: {}",
            estimate.score(),
            estimate.crack_times().offline_slow_hashing_1e4_per_second()
        ))
    }
}

#[cfg(test)]
mod tests {
    mod strength_evaluator_tests {
        use crate::{StrengthEvaluator, ZxcvbnAnalysis};

        #[test]
        fn test_zxcvbn_evaluation() {
            let password = "w".to_string();
            let evaluation = ZxcvbnAnalysis::evaluate(&password).unwrap();
            assert!(evaluation.contains("Score: 0/4"));

            let password = "StrongP@ssw0rdsAreAmazing@#!!!@#$!".to_string();
            let evaluation = ZxcvbnAnalysis::evaluate(&password).unwrap();
            assert!(evaluation.contains("Score: 4/4"));
        }

        #[test]
        fn test_passes_threshold() {
            let weak_password = "weak".to_string();
            assert!(!ZxcvbnAnalysis::passes_threshold(&weak_password).unwrap());

            let strong_password = "StrongP@ssw0rd!".to_string();
            assert!(ZxcvbnAnalysis::passes_threshold(&strong_password).unwrap());
        }
    }
}
