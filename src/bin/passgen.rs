use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Length of the password, if supplied with --max-length, this becomes the minimum length
    #[arg(
        short = 'l',
        long,
        default_value_t = 18,
        long = "length",
        alias = "min-length"
    )]
    min_length: usize,

    /// Maximum password length
    #[arg(long = "max-length")]
    max_length: Option<usize>,

    /// Number of passwords to generate
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

    /// Generate passphrase instead (Supports -w/--words, --seperator and --evaluate)
    #[arg(short = 'p', long)]
    passphrase: bool,

    /// Number of words in the passphrase (only applicable with --passphrase)
    #[arg(short = 'w', long, default_value_t = 4)]
    words: usize,

    /// Separator for words in passphrase (only applicable with --passphrase)
    #[arg(long, default_value = "-")]
    separator: String,

    /// Show password strength evaluation
    #[arg(short = 'e', long = "evaluate-strength")]
    evaluate_strength: bool,
}

fn main() {
    let _cli = Cli::parse();
}
