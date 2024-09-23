use crate::config::PasswordConfig;
use crate::generator::Generator;

pub struct PasswordGenerator;

impl Generator for PasswordGenerator {
    type Config = PasswordConfig;
    type Output = String;

    fn generate(&self, config: &Self::Config) -> Result<Self::Output, Box<dyn std::error::Error>> {
        // Implement password generation logic here
        todo!("Implement password generation")
    }

    fn generate_multiple(
        &self,
        config: &Self::Config,
        amount: usize,
    ) -> Result<Vec<Self::Output>, Box<dyn std::error::Error>> {
        (0..amount).map(|_| self.generate(config)).collect()
    }

    fn evaluate_strength(
        &self,
        output: &Self::Output,
    ) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }
}
