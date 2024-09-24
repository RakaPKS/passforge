pub mod zxcvbn_analysis;

pub use zxcvbn_analysis::ZxcvbnAnalysis;

use std::fmt::Display;

use crate::PassForgeError;

pub trait StrengthEvaluator {
    type Input;
    type Output: Display;

    fn passes_threshold(input: &Self::Input) -> Result<bool, PassForgeError>;
    fn evaluate(input: &Self::Input) -> Result<Self::Output, PassForgeError>;
}
