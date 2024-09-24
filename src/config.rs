use std::ops::RangeInclusive;

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
}

#[derive(Debug, Clone)]
pub struct PassphraseConfig {
    words: usize,
    separator: String,
}

impl PassphraseConfig {
    pub const DEFAULT_WORDS: usize = 6;
    pub const DEFAULT_SEPARATOR: &'static str = "-";

    pub fn new(words: usize, separator: String) -> Self {
        Self { words, separator }
    }
    pub fn builder() -> PassphraseConfigBuilder {
        PassphraseConfigBuilder::default()
    }
}
#[derive(Default)]
pub struct PassphraseConfigBuilder {
    words: Option<usize>,
    separator: Option<String>,
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

    pub fn build(self) -> PassphraseConfig {
        PassphraseConfig {
            words: self.words.unwrap_or(PassphraseConfig::DEFAULT_WORDS),
            separator: self
                .separator
                .unwrap_or(PassphraseConfig::DEFAULT_SEPARATOR.to_string()),
        }
    }
}
