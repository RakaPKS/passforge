use crate::config::PassphraseConfig;
use crate::generator::Generator;

pub struct PassphraseGenerator;

impl Generator for PassphraseGenerator {
    type Config = PassphraseConfig;
    type Output = String;

    fn generate(&self, config: &Self::Config) -> Result<Self::Output, Box<dyn std::error::Error>> {
        // Implement passphrase generation logic here
        todo!("Implement passphrase generation")
    }

    fn generate_multiple(
        &self,
        config: &Self::Config,
        amount: usize,
    ) -> Result<Vec<Self::Output>, Box<dyn std::error::Error>> {
        (0..amount).map(|_| self.generate(config)).collect()
    }
}
