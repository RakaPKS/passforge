//! This module defines the configuration structures for password and passphrase generation.
//!
//! It provides flexible options for customizing the generation process, including
//! length specifications, character set inclusions, word counts, and more.

use std::{ops::RangeInclusive, path::PathBuf};
use rand::Rng;

/// Specifies the word list to use for passphrase generation.
#[derive(Clone, Debug)]
pub enum WordList {
    /// Use the default built-in word list.
    Default,
    /// Use a custom word list from the specified file path.
    Custom(PathBuf),
}

/// Represents preset configurations for quick setup.
#[derive(Clone, Debug)]
pub enum ConfigPreset {
    /// A basic, less secure configuration.
    Weak,
    /// A balanced configuration suitable for most use cases.
    Average,
    /// A highly secure configuration.
    Strong,
}


/// Specifies the length of a password, either as a single value or a range.
#[derive(Debug, Clone)]
pub enum Length {
    /// A single, specific length.
    Single(usize),
    /// A range of acceptable lengths.
    Range(RangeInclusive<usize>),
}


impl Length {
    /// Gets a specific length value. If a range is specified, a random value within the range is returned.
    ///
    /// # Returns
    ///
    /// A `usize` representing the length.
    pub fn get_length(&self) -> usize {
        match self {
            Length::Single(length) => *length,
            Length::Range(range) => {
                rand::thread_rng().gen_range(range.clone())
            }
        }
    }
}

/// Represents the configuration options for password generation.
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    /// The length specification for the password.
    pub length: Length,
    /// Whether to include capital letters in the password.
    pub capitals: bool,
    /// Whether to include numbers in the password.
    pub numbers: bool,
    /// Whether to include symbols in the password.
    pub symbols: bool,
}

impl PasswordConfig {
    /// The default length for generated passwords.
    pub const DEFAULT_LENGTH: usize = 18;
    /// The default setting for including capital letters.
    pub const DEFAULT_CAPITALS: bool = true;
    /// The default setting for including numbers.
    pub const DEFAULT_NUMBERS: bool = true;
    /// The default setting for including symbols.
    pub const DEFAULT_SYMBOLS: bool = true;

    /// Creates a new `PasswordConfig` with the specified options.
    ///
    /// # Arguments
    ///
    /// * `length` - The length specification for the password.
    /// * `capitals` - Whether to include capital letters.
    /// * `numbers` - Whether to include numbers.
    /// * `symbols` - Whether to include symbols.
    pub fn new(length: Length, capitals: bool, numbers: bool, symbols: bool) -> Self {
        Self {
            length,
            capitals,
            numbers,
            symbols,
        }
    }

    /// Returns a new `PasswordConfigBuilder` for creating a `PasswordConfig`.
    pub fn builder() -> PasswordConfigBuilder {
        PasswordConfigBuilder::default()
    }
}

/// A builder for creating `PasswordConfig` instances.
#[derive(Default)]
pub struct PasswordConfigBuilder {
    length: Option<Length>,
    capitals: Option<bool>,
    numbers: Option<bool>,
    symbols: Option<bool>,
}

impl PasswordConfigBuilder {
    /// Sets the length for the password.
    pub fn length(mut self, length: Length) -> Self {
        self.length = Some(length);
        self
    }

    /// Sets whether to include capital letters.
    pub fn capitals(mut self, include: bool) -> Self {
        self.capitals = Some(include);
        self
    }

    /// Sets whether to include numbers.
    pub fn numbers(mut self, include: bool) -> Self {
        self.numbers = Some(include);
        self
    }

    /// Sets whether to include symbols.
    pub fn symbols(mut self, include: bool) -> Self {
        self.symbols = Some(include);
        self
    }

    /// Builds a `PasswordConfig` from the current builder state.
    pub fn build(self) -> PasswordConfig {
        PasswordConfig {
            length: self
                .length
                .unwrap_or(Length::Single(PasswordConfig::DEFAULT_LENGTH)),
            capitals: self.capitals.unwrap_or(PasswordConfig::DEFAULT_CAPITALS),
            numbers: self.numbers.unwrap_or(PasswordConfig::DEFAULT_NUMBERS),
            symbols: self.symbols.unwrap_or(PasswordConfig::DEFAULT_SYMBOLS),
        }
    }

    /// Builds a `PasswordConfig` from a preset configuration.
    pub fn build_from_preset(self, preset: ConfigPreset) -> PasswordConfig {
        match preset {
            ConfigPreset::Weak => PasswordConfig {
                length: Length::Single(8),
                capitals: true,
                numbers: true,
                symbols: false,
            },
            ConfigPreset::Average => PasswordConfig {
                length: Length::Single(16),
                capitals: true,
                numbers: true,
                symbols: true,
            },
            ConfigPreset::Strong => PasswordConfig {
                length: Length::Single(32),
                capitals: true,
                numbers: true,
                symbols: true,
            },
        }
    }
}

/// Represents the configuration options for passphrase generation.
#[derive(Debug, Clone)]
pub struct PassphraseConfig {
    /// The number of words to include in the passphrase.
    pub words: usize,
    /// The separator to use between words.
    pub separator: String,
    /// The word list to use for generating the passphrase.
    pub word_list: WordList,
}

impl PassphraseConfig {
    /// The default number of words for generated passphrases.
    pub const DEFAULT_WORDS: usize = 6;
    /// The default separator for generated passphrases.
    pub const DEFAULT_SEPARATOR: &'static str = "-";

    /// Creates a new `PassphraseConfig` with the specified options.
    ///
    /// # Arguments
    ///
    /// * `words` - The number of words to include in the passphrase.
    /// * `separator` - The separator to use between words.
    /// * `word_list` - The word list to use for generating the passphrase.
    pub fn new(words: usize, separator: String, word_list: WordList) -> Self {
        Self {
            words,
            separator,
            word_list,
        }
    }

    /// Returns a new `PassphraseConfigBuilder` for creating a `PassphraseConfig`.
    pub fn builder() -> PassphraseConfigBuilder {
        PassphraseConfigBuilder::default()
    }
}

/// A builder for creating `PassphraseConfig` instances.
#[derive(Default)]
pub struct PassphraseConfigBuilder {
    words: Option<usize>,
    separator: Option<String>,
    word_list: Option<WordList>,
}

impl PassphraseConfigBuilder {
    /// Sets the number of words for the passphrase.
    pub fn words(mut self, count: usize) -> Self {
        self.words = Some(count);
        self
    }

    /// Sets the separator for the passphrase.
    pub fn separator(mut self, sep: String) -> Self {
        self.separator = Some(sep);
        self
    }

    /// Sets the word list for the passphrase.
    pub fn word_list(mut self, wl: WordList) -> Self {
        self.word_list = Some(wl);
        self
    }

    /// Builds a `PassphraseConfig` from the current builder state.
    pub fn build(self) -> PassphraseConfig {
        PassphraseConfig {
            words: self.words.unwrap_or(PassphraseConfig::DEFAULT_WORDS),
            separator: self
                .separator
                .unwrap_or(PassphraseConfig::DEFAULT_SEPARATOR.to_string()),
            word_list: self.word_list.unwrap_or(WordList::Default),
        }
    }

    /// Builds a `PassphraseConfig` from a preset configuration.
    pub fn build_from_preset(self, preset: ConfigPreset) -> PassphraseConfig {
        match preset {
            ConfigPreset::Weak => PassphraseConfig {
                words: 4,
                separator: "-".into(),
                word_list: WordList::Default,
            },
            ConfigPreset::Average => PassphraseConfig {
                words: 8,
                separator: "-".into(),
                word_list: WordList::Default,
            },
            ConfigPreset::Strong => PassphraseConfig {
                words: 16,
                separator: "-".into(),
                word_list: WordList::Default,
            },
        }
    }
}