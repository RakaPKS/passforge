pub mod zxcvbn_analysis;

pub use zxcvbn_analysis::ZxcvbnAnalysis;

use std::fmt::Display;

pub trait StrengthEvaluator {
    type Input;
    type Output: Display;

    fn passes_threshold(input: &Self::Input) -> Result<bool, Box<dyn std::error::Error>>;
    fn evaluate(input: &Self::Input) -> Result<Self::Output, Box<dyn std::error::Error>>;
}
