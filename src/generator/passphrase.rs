//! This module implements passphrase generation functionality.
//!
//! It provides a `PassphraseGenerator` struct that implements the `Generator` trait,
//! allowing for flexible and customizable passphrase generation using word lists.

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::config::{PassphraseConfig, WordList};
use crate::generator::Generator;
use crate::PassForgeError;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// The default word list used for passphrase generation.
const DEFAULT_WORD_LIST: &str = include_str!("../../resources/eff_large_wordlist.txt");

/// Struct for generating passphrases based on specified configurations.
pub struct PassphraseGenerator;

impl PassphraseGenerator {
    /// Creates a passphrase from the given word list and configuration.
    ///
    /// # Arguments
    ///
    /// * `word_list` - A vector of words to choose from.
    /// * `words` - The number of words to include in the passphrase.
    /// * `separator` - The string used to separate words in the passphrase.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the generated passphrase as a `String` if successful,
    /// or a `PassForgeError` if an error occurred during generation.
    fn create_passphrase(
        word_list: &Vec<String>,
        words: usize,
        separator: &String,
    ) -> Result<String, PassForgeError> {
        let mut rng = thread_rng();

        let passphrase_words: Vec<&str> = word_list
            .choose_multiple(&mut rng, words)
            .map(String::as_str)
            .collect();

        Ok(passphrase_words.join(separator))
    }

    /// Loads and processes the word list based on the specified `WordList` type.
    ///
    /// # Arguments
    ///
    /// * `word_list` - A reference to the `WordList` enum specifying the source of words.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of words if successful,
    /// or a `PassForgeError` if an error occurred during loading or processing.
    fn get_word_list(word_list: &WordList) -> Result<Vec<String>, PassForgeError> {
        let words: Vec<String> = PassphraseGenerator::load_file(word_list)?
            .into_iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                match parts.len() {
                    1 => Some(parts[0].to_string()),
                    2 => parts[1].parse().ok(),
                    _ => None, // Invalid format, skip this line
                }
            })
            .collect();
        if words.is_empty() {
            return Err(PassForgeError::WordListError(
                "Word list is empty or invalid".into(),
            ));
        }
        Ok(words)
    }

    /// Loads the word list file into memory.
    ///
    /// # Arguments
    ///
    /// * `word_list` - A reference to the `WordList` enum specifying the source of words.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of lines from the file if successful,
    /// or a `PassForgeError` if an error occurred during file reading.
    fn load_file(word_list: &WordList) -> Result<Vec<String>, PassForgeError> {
        let line = match word_list {
            WordList::Default => DEFAULT_WORD_LIST.lines().map(String::from).collect(),
            WordList::Custom(path) => {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                reader.lines().map_while(Result::ok).collect()
            }
        };
        Ok(line)
    }
}

impl Generator for PassphraseGenerator {
    type Config = PassphraseConfig;
    type Output = String;

    /// Generates a single passphrase based on the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the `PassphraseConfig` specifying generation parameters.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the generated passphrase as a `String` if successful,
    /// or a `PassForgeError` if an error occurred during generation.
    ///
    /// # Errors
    ///
    /// Will return an error if the specified number of words is less than or equal to 1.
    fn generate(config: &Self::Config) -> Result<Self::Output, PassForgeError> {
        if config.words <= 1 {
            return Err(PassForgeError::InvalidWordCount(
                "Amount of words cannot be smaller than 1".into(),
            ));
        }
        let word_list = PassphraseGenerator::get_word_list(&config.word_list)?;
        PassphraseGenerator::create_passphrase(&word_list, config.words, &config.separator)
    }

    /// Generates multiple passphrases based on the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the `PassphraseConfig` specifying generation parameters.
    /// * `amount` - The number of passphrases to generate.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of generated passphrases as `String`s if successful,
    /// or a `PassForgeError` if an error occurred during generation.
    ///
    /// # Errors
    ///
    /// Will return an error if the specified amount is less than or equal to 1,
    /// or if the specified number of words per passphrase is less than or equal to 1.
    fn generate_multiple(
        config: &Self::Config,
        amount: usize,
    ) -> Result<Vec<Self::Output>, PassForgeError> {
        if amount <= 1 {
            return Err(PassForgeError::InvalidGenAmount(
                "Amount cannot be smaller than 1".into(),
            ));
        }
        if config.words <= 1 {
            return Err(PassForgeError::InvalidWordCount(
                "Amount of words cannot be smaller than 1".into(),
            ));
        }
        let word_list = PassphraseGenerator::get_word_list(&config.word_list)?;

        (0..amount)
            .map(|_| {
                PassphraseGenerator::create_passphrase(&word_list, config.words, &config.separator)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod passphrase_generator_tests {
        use super::*;

        #[test]
        fn test_passphrase_generation() {
            let config = PassphraseConfig::new(4, "-".to_string(), WordList::Default);
            let passphrase = PassphraseGenerator::generate(&config).unwrap();
            let words: Vec<&str> = passphrase.split('-').collect();
            assert_eq!(words.len(), 4);
        }

        #[test]
        fn test_passphrase_generation_custom_separator() {
            let config = PassphraseConfig::new(4, "_".to_string(), WordList::Default);
            let passphrase = PassphraseGenerator::generate(&config).unwrap();
            let words: Vec<&str> = passphrase.split('_').collect();
            assert_eq!(words.len(), 4);
        }

        #[test]
        fn test_generate_multiple_passphrases() {
            let config = PassphraseConfig::new(4, "-".to_string(), WordList::Default);
            let passphrases = PassphraseGenerator::generate_multiple(&config, 5).unwrap();
            assert_eq!(passphrases.len(), 5);
            for passphrase in passphrases {
                let words: Vec<&str> = passphrase.split('-').collect();
                assert_eq!(words.len(), 4);
            }
        }

        #[test]
        fn test_invalid_word_count() {
            let config = PassphraseConfig::new(0, "-".to_string(), WordList::Default);
            assert!(PassphraseGenerator::generate(&config).is_err());
        }
    }
}
