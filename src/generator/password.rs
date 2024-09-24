use rand::Rng;

use crate::config::PasswordConfig;
use crate::generator::Generator;

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

    fn generate(&self, config: &Self::Config) -> Result<Self::Output, Box<dyn std::error::Error>> {
        // Define character sets as byte slices for memory efficiency

        let mut rng = rand::thread_rng();
        let length = config.length.get_length();

        if length < 1 {
            return Err("Length of password cannot be less than 1".into());
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
        &self,
        config: &Self::Config,
        amount: usize,
    ) -> Result<Vec<Self::Output>, Box<dyn std::error::Error>> {
        if amount <= 1 {
            return Err("Amount cannot be smaller than 1".into());
        }
        (0..amount).map(|_| self.generate(config)).collect()
    }
}
