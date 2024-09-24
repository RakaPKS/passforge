pub mod passphrase;
pub mod password;

pub use passphrase::PassphraseGenerator;
pub use password::PasswordGenerator;

use crate::PassGenError;

pub trait Generator {
    type Config;
    type Output;

    fn generate(config: &Self::Config) -> Result<Self::Output, PassGenError>;
    fn generate_multiple(
        config: &Self::Config,
        amount: usize,
    ) -> Result<Vec<Self::Output>, PassGenError>;
}
