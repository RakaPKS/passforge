pub mod passphrase;
pub mod password;

pub use passphrase::PassphraseGenerator;
pub use password::PasswordGenerator;

pub trait Generator {
    type Config;
    type Output;

    fn generate(config: &Self::Config) -> Result<Self::Output, Box<dyn std::error::Error>>;
    fn generate_multiple(
        config: &Self::Config,
        amount: usize,
    ) -> Result<Vec<Self::Output>, Box<dyn std::error::Error>>;
}
