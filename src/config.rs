use std::{ops::RangeInclusive, path::PathBuf};

#[derive(Debug, Clone)]
pub struct PasswordConfig {
    pub length: Length,
    pub capitals: bool,
    pub numbers: bool,
    pub symbols: bool,
}

#[derive(Debug, Clone)]
pub enum Length {
    Single(usize),
    Range(RangeInclusive<usize>),
}

impl Length {
    pub fn get_length(&self) -> usize {
        match self {
            Length::Single(length) => *length,
            Length::Range(range) => {
                use rand::Rng;
                rand::thread_rng().gen_range(range.clone())
            }
        }
    }
}

impl PasswordConfig {
    pub const DEFAULT_LENGTH: usize = 18;
    pub const DEFAULT_CAPITALS: bool = true;
    pub const DEFAULT_NUMBERS: bool = true;
    pub const DEFAULT_SYMBOLS: bool = true;

    pub fn new(length: Length, capitals: bool, numbers: bool, symbols: bool) -> Self {
        Self {
            length,
            capitals,
            numbers,
            symbols,
        }
    }
    pub fn builder() -> PasswordConfigBuilder {
        PasswordConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct PasswordConfigBuilder {
    length: Option<Length>,
    capitals: Option<bool>,
    numbers: Option<bool>,
    symbols: Option<bool>,
}

impl PasswordConfigBuilder {
    pub fn length(mut self, length: Length) -> Self {
        self.length = Some(length);
        self
    }

    pub fn capitals(mut self, include: bool) -> Self {
        self.capitals = Some(include);
        self
    }

    pub fn numbers(mut self, include: bool) -> Self {
        self.numbers = Some(include);
        self
    }

    pub fn symbols(mut self, include: bool) -> Self {
        self.symbols = Some(include);
        self
    }

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

#[derive(Debug, Clone)]
pub struct PassphraseConfig {
    pub words: usize,
    pub separator: String,
    pub word_list: WordList,
}

impl PassphraseConfig {
    pub const DEFAULT_WORDS: usize = 6;
    pub const DEFAULT_SEPARATOR: &'static str = "-";

    pub fn new(words: usize, separator: String, word_list: WordList) -> Self {
        Self {
            words,
            separator,
            word_list,
        }
    }
    pub fn builder() -> PassphraseConfigBuilder {
        PassphraseConfigBuilder::default()
    }
}
#[derive(Default)]
pub struct PassphraseConfigBuilder {
    words: Option<usize>,
    separator: Option<String>,
    word_list: Option<WordList>,
}

impl PassphraseConfigBuilder {
    pub fn words(mut self, count: usize) -> Self {
        self.words = Some(count);
        self
    }

    pub fn separator(mut self, sep: String) -> Self {
        self.separator = Some(sep);
        self
    }

    pub fn word_list(mut self, wl: WordList) -> Self {
        self.word_list = Some(wl);
        self
    }

    pub fn build(self) -> PassphraseConfig {
        PassphraseConfig {
            words: self.words.unwrap_or(PassphraseConfig::DEFAULT_WORDS),
            separator: self
                .separator
                .unwrap_or(PassphraseConfig::DEFAULT_SEPARATOR.to_string()),
            word_list: self.word_list.unwrap_or(WordList::Default),
        }
    }

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

#[derive(Clone, Debug)]
pub enum WordList {
    Default,
    Custom(PathBuf),
}

#[derive(Clone, Debug)]
pub enum ConfigPreset {
    Weak,
    Average,
    Strong,
}
