
//! PassForge CLI
//!
//! This is the main entry point for the PassForge command-line interface.
//! It parses command-line arguments and calls the appropriate PassForge
//! library functions to generate passwords or passphrases.

use std::{fmt::Display, path::PathBuf, process};

use clap::Parser;
use passforge::{
    config::{ConfigPreset, PassphraseConfigBuilder, PasswordConfigBuilder},
    Generator, Length, PassForgeError, PassphraseConfig, PassphraseGenerator, PasswordConfig,
    PasswordGenerator, StrengthEvaluator, WordList, ZxcvbnAnalysis,
};

/// CLI argument structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Length of the password, if supplied with --max-length, this becomes the minimum length.
    /// Default = 18
    #[arg(
        short = 'l',
        long,
        default_value_t = PasswordConfig::DEFAULT_LENGTH,
        long = "length",
        alias = "min-length"
    )]
    min_length: usize,

    /// Maximum password length
    #[arg(long = "max-length")]
    max_length: Option<usize>,

    /// Number of passwords to generate. Default = 1
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Exclude uppercase letters from the password
    #[arg(short = 'u', long = "no-capitals", alias = "nc")]
    no_capitals: bool,

    /// Exclude numeric values from the password
    #[arg(short = 'n', long = "no-numbers", alias = "nn")]
    no_numbers: bool,

    /// Exclude symbols from the password
    #[arg(short = 's', long = "no-symbols", alias = "ns")]
    no_symbols: bool,

    /// Generate passphrase instead (Supports -c/--count -w/--words, --seperator --word-list and --evaluate)
    #[arg(short = 'p', long)]
    passphrase: bool,

    /// Number of words in the passphrase (only applicable with --passphrase). Default = 4
    #[arg(short = 'w', long, default_value_t = PassphraseConfig::DEFAULT_WORDS)]
    words: usize,

    /// Separator for words in passphrase (only applicable with --passphrase)
    #[arg(long, default_value = PassphraseConfig::DEFAULT_SEPARATOR)]
    separator: String,

    /// Path to a custom word list file for passphrase generation
    #[arg(long = "word-list", value_name = "FILE")]
    word_list: Option<PathBuf>,

    /// Show password strength evaluation
    #[arg(short = 'e', long = "evaluate-strength")]
    evaluate_strength: bool,

    /// Preset for quick generation, disables all flags aside --passhrase/-p and
    /// -e/--evaluate-strength. Choices: Weak, Average, Strong
    #[arg(long = "preset")]
    preset: Option<String>,
}

// Helper functions
fn parse_preset(preset_str: &str) -> Result<ConfigPreset, PassForgeError> {
    match preset_str.to_lowercase().as_str() {
        "weak" => Ok(ConfigPreset::Weak),
        "average" => Ok(ConfigPreset::Average),
        "strong" => Ok(ConfigPreset::Strong),
        _ => Err(PassForgeError::InvalidConfig(
            "Invalid preset. Choices are: Weak, Average, Strong".into(),
        )),
    }
}

fn parse_length(min: usize, max: Option<usize>) -> Result<Length, PassForgeError> {
    match max {
        Some(max) if max > min => Ok(Length::Range(min..=max)),
        Some(max) if max == min => Ok(Length::Single(min)),
        Some(_) => Err(PassForgeError::InvalidLength(
            "Maximum length must be greater than or equal to minimum length".into(),
        )),
        None => Ok(Length::Single(min)),
    }
}

// Main generation functions
fn gen_password(input: Cli) -> Result<(), PassForgeError> {
    let config = if let Some(preset_str) = input.preset {
        let preset = parse_preset(&preset_str)?;
        PasswordConfigBuilder::default().build_from_preset(preset)
    } else {
        let length = parse_length(input.min_length, input.max_length)?;
        PasswordConfig::new(
            length,
            !input.no_capitals,
            !input.no_numbers,
            !input.no_symbols,
        ) 
    };

    let generator = PasswordGenerator;
    let strength_evaluator = ZxcvbnAnalysis;
    generate_items(
        &generator,
        &config,
        input.count,
        input.evaluate_strength,
        &strength_evaluator,
    )
}

fn gen_passphrase(input: Cli) -> Result<(), PassForgeError> {
    let config = if let Some(preset_str) = input.preset {
        let preset = parse_preset(&preset_str)?;
        PassphraseConfigBuilder::default().build_from_preset(preset)
    } else {
        let word_list = match input.word_list {
            Some(path) => WordList::Custom(path),
            None => WordList::Default,
        };
        PassphraseConfig::new(input.words, input.separator, word_list)
    };

    let generator = PassphraseGenerator;
    let strength_evaluator = ZxcvbnAnalysis;
    generate_items(
        &generator,
        &config,
        input.count,
        input.evaluate_strength,
        &strength_evaluator,
    )
}

fn generate_items<G, S>(
    _: &G,
    config: &G::Config,
    count: usize,
    evaluate_strength: bool,
    _: &S,
) -> Result<(), PassForgeError>
where
    G: Generator,
    G::Output: Display,
    S: StrengthEvaluator<Input = String>,
    S::Output: Display,
{
    let items = match count {
        0 => {
            return Err(PassForgeError::InvalidGenAmount(
                "Count cannot be smaller than 1".into(),
            ))
        }
        1 => vec![G::generate(config)?],
        _ => G::generate_multiple(config, count)?,
    };

    for item in items {
        println!("{}", item);
        if evaluate_strength {
            match item.to_string().parse() {
                Ok(password) => match S::evaluate(&password) {
                    Ok(evaluation) => println!("Strength: {}", evaluation),
                    Err(e) => eprintln!("Error evaluating strength: {}", e),
                },
                Err(_) => eprintln!("Unable to evaluate strength for this type of output"),
            }
        }
    }

    Ok(())
}



fn main() {
    let cli = Cli::parse();

    let result = if cli.passphrase {
        gen_passphrase(cli)
    } else {
        gen_password(cli)
    };

    match result {
        Ok(_) => process::exit(0),
        Err(msg) => {
            eprintln!("Error: {}", msg);
            process::exit(1)
        }
    }
}
