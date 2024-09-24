use rand::Rng;

use crate::config::PasswordConfig;
use crate::generator::Generator;
use crate::PassGenError;

pub struct PasswordGenerator;

impl PasswordGenerator {
    const LOWERCASE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &'static [u8] = b"0123456789";
    const SYMBOLS: &'static [u8] = b"!@#$%^&*()-_=+[]{}|;:,.<>?";
}

impl Generator for PasswordGenerator {
    type Config = PasswordConfig;
    type Output = String;

    fn generate(config: &Self::Config) -> Result<Self::Output, PassGenError> {
        // Define character sets as byte slices for memory efficiency

        let mut rng = rand::thread_rng();
        let length = config.length.get_length();

        if length < 1 {
            return Err(PassGenError::InvalidLength(
                "Length of password cannot be less than 1".into(),
            ));
        }

        // Calculate total character set length to pre-allocate memory
        let mut total_len = Self::LOWERCASE.len();
        if config.capitals {
            total_len += Self::UPPERCASE.len();
        }
        if config.numbers {
            total_len += Self::NUMBERS.len();
        }
        if config.symbols {
            total_len += Self::SYMBOLS.len();
        }

        // Create a single Vec<u8> with all allowed characters
        let mut chars = Vec::with_capacity(total_len);
        chars.extend_from_slice(Self::LOWERCASE);
        if config.capitals {
            chars.extend_from_slice(Self::UPPERCASE);
        }
        if config.numbers {
            chars.extend_from_slice(Self::NUMBERS);
        }
        if config.symbols {
            chars.extend_from_slice(Self::SYMBOLS);
        }

        // Generate password using byte operations for efficiency
        let result: String = (0..length)
            .map(|_| chars[rng.gen_range(0..chars.len())] as char)
            .collect();
        Ok(result)
    }

    fn generate_multiple(
        config: &Self::Config,
        amount: usize,
    ) -> Result<Vec<Self::Output>, PassGenError> {
        if amount <= 1 {
            return Err(PassGenError::InvalidGenAmount(
                "Amount cannot be smaller than 1".into(),
            ));
        }
        (0..amount)
            .map(|_| PasswordGenerator::generate(config))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod password_generator_tests {
        use crate::Length;

        use super::*;

        #[test]
        fn test_password_generation() {
            let config = PasswordConfig::new(Length::Single(16), true, true, true);
            let password = PasswordGenerator::generate(&config).unwrap();
            assert_eq!(password.len(), 16);
        }

        #[test]
        fn test_password_generation_no_capitals() {
            let config = PasswordConfig::new(Length::Single(16), false, true, true);
            let password = PasswordGenerator::generate(&config).unwrap();
            assert_eq!(password.len(), 16);
            assert!(!password.chars().any(|c| c.is_uppercase()));
        }

        #[test]
        fn test_password_generation_no_numbers() {
            let config = PasswordConfig::new(Length::Single(16), true, false, true);
            let password = PasswordGenerator::generate(&config).unwrap();
            assert_eq!(password.len(), 16);
            assert!(!password.chars().any(|c| c.is_numeric()));
        }

        #[test]
        fn test_password_generation_no_symbols() {
            let config = PasswordConfig::new(Length::Single(16), true, true, false);
            let password = PasswordGenerator::generate(&config).unwrap();
            assert_eq!(password.len(), 16);
            assert!(!password
                .chars()
                .any(|c| "!@#$%^&*()-_=+[]{}|;:,.<>?".contains(c)));
        }

        #[test]
        fn test_password_generation_range_length() {
            let config = PasswordConfig::new(Length::Range(10..=20), true, true, true);
            let password = PasswordGenerator::generate(&config).unwrap();
            assert!(password.len() >= 10 && password.len() <= 20);
        }

        #[test]
        fn test_generate_multiple_passwords() {
            let config = PasswordConfig::new(Length::Single(16), true, true, true);
            let passwords = PasswordGenerator::generate_multiple(&config, 5).unwrap();
            assert_eq!(passwords.len(), 5);
            for password in passwords {
                assert_eq!(password.len(), 16);
            }
        }

        #[test]
        fn test_invalid_length() {
            let config = PasswordConfig::new(Length::Single(0), true, true, true);
            assert!(PasswordGenerator::generate(&config).is_err());
        }
    }
}
