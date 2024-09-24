use std::fmt::Display;

use clap::Parser;
use passgen::{
    Generator, Length, PassphraseConfig, PassphraseGenerator, PasswordConfig, PasswordGenerator,
    StrengthEvaluator, ZxcvbnAnalysis,
};

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

    /// Generate passphrase instead (Supports -c/--count -w/--words, --seperator and --evaluate)
    #[arg(short = 'p', long)]
    passphrase: bool,

    /// Number of words in the passphrase (only applicable with --passphrase). Default = 4
    #[arg(short = 'w', long, default_value_t = PassphraseConfig::DEFAULT_WORDS)]
    words: usize,

    /// Separator for words in passphrase (only applicable with --passphrase)
    #[arg(long, default_value = PassphraseConfig::DEFAULT_SEPARATOR)]
    separator: String,

    /// Show password strength evaluation
    #[arg(short = 'e', long = "evaluate-strength")]
    evaluate_strength: bool,
}

fn generate_items<G, S>(
    generator: &G,
    config: &G::Config,
    count: usize,
    evaluate_strength: bool,
    _strength_evaluator: &S,
) -> Result<(), Box<dyn std::error::Error>>
where
    G: Generator,
    G::Output: Display,
    S: StrengthEvaluator<Input = String>,
    S::Output: Display,
{
    let items = if count > 1 {
        generator.generate_multiple(config, count)?
    } else if count == 1 {
        vec![generator.generate(config)?]
    } else {
        return Err("Count cannot be smaller than 1".into());
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
fn gen_password(input: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let length = match input.max_length {
        Some(max) if max > input.min_length => Length::Range(input.min_length..=max),
        Some(max) if max == input.min_length => Length::Single(input.min_length),
        Some(_) => {
            return Err("Maximum length must be greater than or equal to minimum length".into())
        }
        None => Length::Single(input.min_length),
    };

    let config = PasswordConfig::new(
        length,
        !input.no_capitals,
        !input.no_numbers,
        !input.no_symbols,
    );

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

fn gen_passphrase(input: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let config = PassphraseConfig::new(input.words, input.separator);

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.passphrase {
        gen_passphrase(cli)
    } else {
        gen_password(cli)
    }
}
