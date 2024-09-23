pub mod passphrase;
pub mod password;

use std::fmt::Display;

pub use passphrase::PassphraseGenerator;
pub use password::PasswordGenerator;

pub trait Generator {
    type Config;
    type Output: Display;

    fn generate(&self, config: &Self::Config) -> Result<Self::Output, Box<dyn std::error::Error>>;
    fn generate_multiple(
        &self,
        config: &Self::Config,
        amount: usize,
    ) -> Result<Vec<Self::Output>, Box<dyn std::error::Error>>;
    fn evaluate_strength(
        &self,
        output: &Self::Output,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
