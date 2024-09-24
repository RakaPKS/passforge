use zxcvbn::zxcvbn;

use crate::strength_evaluator::StrengthEvaluator;

pub struct ZxcvbnAnalysis;

impl ZxcvbnAnalysis {
    const MIN_PASS_SCORE: u8 = 3;
}

impl StrengthEvaluator for ZxcvbnAnalysis {
    type Input = String;
    type Output = String;

    fn passes_threshold(input: &Self::Input) -> Result<bool, Box<dyn std::error::Error>> {
        if input.is_empty() {
            return Err("Password cannot be empty".into());
        }
        Ok((zxcvbn(input, &[]).score() as u8) >= Self::MIN_PASS_SCORE)
    }

    fn evaluate(input: &Self::Input) -> Result<Self::Output, Box<dyn std::error::Error>> {
        if input.is_empty() {
            return Err("Input password cannot be empty".into());
        }
        let estimate = zxcvbn(input, &[]);
        Ok(format!(
            "Score: {}/4, Crack time: {}",
            estimate.score(),
            estimate.crack_times().offline_slow_hashing_1e4_per_second()
        ))
    }
}
