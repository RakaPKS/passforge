//! This module defines the core generation functionality for passwords and passphrases.
//!
//! It provides a common `Generator` trait that can be implemented by different types of
//! generators, allowing for a flexible and extensible password generation system.

use crate::PassForgeError;

/// The `Generator` trait defines the interface for password and passphrase generation.
///
/// This trait allows for a common interface across different types of generators,
/// enabling easy swapping and extension of generation algorithms.
pub trait Generator {
    /// The configuration type used by this generator.
    type Config;
    /// The output type produced by this generator.
    type Output;

    /// Generates a single item (password or passphrase) based on the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the configuration specifying generation parameters.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the generated item if successful, or a `PassForgeError` if an error occurred.
    fn generate(config: &Self::Config) -> Result<Self::Output, PassForgeError>;

    /// Generates multiple items (passwords or passphrases) based on the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the configuration specifying generation parameters.
    /// * `amount` - The number of items to generate.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of generated items if successful, or a `PassForgeError` if an error occurred.
    fn generate_multiple(
        config: &Self::Config,
        amount: usize,
    ) -> Result<Vec<Self::Output>, PassForgeError>;
}

pub mod passphrase;
pub mod password;

pub use passphrase::PassphraseGenerator;
pub use password::PasswordGenerator;
