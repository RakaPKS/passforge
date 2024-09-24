use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::config::{PassphraseConfig, WordList};
use crate::generator::Generator;
use crate::PassGenError;
use rand::seq::SliceRandom;
use rand::thread_rng;

const DEFAULT_WORD_LIST: &str = include_str!("../../resources/eff_large_wordlist.txt");

pub struct PassphraseGenerator;

impl PassphraseGenerator {
    fn create_passphrase(
        word_list: &Vec<String>,
        words: usize,
        separator: &String,
    ) -> Result<String, PassGenError> {
        let mut rng = thread_rng();

        let passphrase_words: Vec<&str> = word_list
            .choose_multiple(&mut rng, words)
            .map(String::as_str)
            .collect();

        Ok(passphrase_words.join(separator))
    }

    fn get_word_list(word_list: &WordList) -> Result<Vec<String>, PassGenError> {
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
            return Err(PassGenError::WordListError(
                "Word list is empty or invalid".into(),
            ));
        }
        Ok(words)
    }
    fn load_file(word_list: &WordList) -> Result<Vec<String>, PassGenError> {
        let line = match word_list {
            WordList::Default => DEFAULT_WORD_LIST.lines().map(String::from).collect(),
            WordList::Custom(path) => {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                reader.lines().filter_map(Result::ok).collect()
            }
        };
        Ok(line)
    }
}

impl Generator for PassphraseGenerator {
    type Config = PassphraseConfig;
    type Output = String;

    fn generate(config: &Self::Config) -> Result<Self::Output, PassGenError> {
        if config.words <= 1 {
            return Err(PassGenError::InvalidGenAmount(
                "Amount of words cannot be smaller than 1".into(),
            ));
        }
        let word_list = PassphraseGenerator::get_word_list(&config.word_list)?;
        PassphraseGenerator::create_passphrase(&word_list, config.words, &config.separator)
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
        if config.words <= 1 {
            return Err(PassGenError::InvalidWordCount(
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
