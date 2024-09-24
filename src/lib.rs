//! # PassForge
//!
//! `passforge` is a robust and flexible password generation library that allows users to create
//! secure passwords and passphrases with various customization options. It also provides
//! functionality for evaluating password strength and allows for easy extension through the
//! `Generator` and `StrengthEvaluator`.
//!
//! ## Features
//!
//! - Generate passwords with customizable length and character sets
//! - Create passphrases using a word list
//! - Evaluate password strength using the zxcvbn algorithm
//! - Command-line interface for easy use
//! - Extendible through `Generator` and `StrengthEvaluator` traits.
//!
//! ## Getting Started
//!
//! To use PassForge in your project, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! passforge = "0.1.0"
//! ```
//!
//! Then, you can use the library in your Rust code as shown in the examples below.
//!
//! ## Examples
//!
//! Generating a Password
//!
//! ```
//! use passforge::{PasswordConfig, PasswordGenerator, Generator, Length};
//!
//! let config = PasswordConfig::new(Length::Single(16), true, true, true);
//! let password = PasswordGenerator::generate(&config).expect("Failed to generate password");
//! println!("Generated password: {}", password);
//! ```
//!
//! Creating a Passphrase
//!
//! ```
//! use passforge::{PassphraseConfig, PassphraseGenerator, Generator, WordList};
//!
//! let config = PassphraseConfig::new(4, "-".to_string(), WordList::Default);
//! let passphrase = PassphraseGenerator::generate(&config).expect("Failed to generate passphrase");
//! println!("Generated passphrase: {}", passphrase);
//! ```
//!
//! Evaluating Password Strength
//!
//! ```
//! use passforge::{ZxcvbnAnalysis, StrengthEvaluator};
//!
//! let password = "example_password".into();
//! let strength = ZxcvbnAnalysis::evaluate(&password).expect("Failed to evaluate password");
//! println!("Password strength: {}", strength);
//! ```

// Re-export main structs and traits for easier access
pub use config::{
    Length, PassphraseConfig, PassphraseConfigBuilder, PasswordConfig, PasswordConfigBuilder,
    WordList,
};
pub use error::PassForgeError;
pub use generator::{Generator, PassphraseGenerator, PasswordGenerator};
pub use strength_evaluator::{StrengthEvaluator, ZxcvbnAnalysis};

/// Configuration structures for password and passphrase generation,
pub mod config;

/// Custom error types used throughout the crate to provide
/// detailed information about failure conditions.
pub mod error;

/// Core generation functionality for passwords and passphrases,
/// implementing the Generator trait for different types of generators.
pub mod generator;

/// Password strength evaluation functionality using the zxcvbn algorithm,
/// providing detailed analysis of password security. Extendible by implementing the
/// `StrengthEvaluator trait`
pub mod strength_evaluator;
