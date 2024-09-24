use zxcvbn::zxcvbn;

use crate::{strength_evaluator::StrengthEvaluator, PassGenError};

pub struct ZxcvbnAnalysis;

impl ZxcvbnAnalysis {
    const MIN_PASS_SCORE: u8 = 3;
}

impl StrengthEvaluator for ZxcvbnAnalysis {
    type Input = String;
    type Output = String;

    fn passes_threshold(input: &Self::Input) -> Result<bool, PassGenError> {
        if input.is_empty() {
            return Err(PassGenError::InvalidLength(
                "Password cannot be empty".into(),
            ));
        }
        Ok((zxcvbn(input, &[]).score() as u8) >= Self::MIN_PASS_SCORE)
    }

    fn evaluate(input: &Self::Input) -> Result<Self::Output, PassGenError> {
        if input.is_empty() {
            return Err(PassGenError::InvalidLength(
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
