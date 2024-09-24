# PassForge: CLI Password Generator

PassForge is a robust and flexible command-line interface (CLI) password generation tool built with Rust. It allows users to create secure passwords and passphrases with various customization options.

## Features

- Generate passwords with customizable length and character sets
- Create passphrases using a word list
- Evaluate password strength using the zxcvbn algorithm
- Multiple output options (single or multiple passwords/passphrases)
- Preset configurations for quick generation
- Extendable through `Generator` and `StrengthEvaluator` traits

## Installation

To install PassForge, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [rustup.rs](https://rustup.rs/).

Once you have Rust and Cargo installed, you can build and install PassForge using the following commands:

```bash
git clone https://github.com/RakaPKS/passforge.git
cd passforge
cargo install --path .
```

This will install the `passforge` binary in your Cargo bin directory.

## Usage

Here are some examples of how to use PassForge:

1. Generate a single password with default settings:
   ```
   passforge
   ```

2. Generate a password with a specific length:
   ```
   passforge --length 20
   ```

3. Generate a password without symbols:
   ```
   passforge --no-symbols
   ```

4. Generate multiple passwords:
   ```
   passforge --count 5
   ```

5. Generate a passphrase:
   ```
   passforge --passphrase
   ```

6. Generate a passphrase with custom word count and separator:
   ```
   passforge --passphrase --words 5 --separator "_"
   ```

7. Use a preset configuration:
   ```
   passforge --preset strong
   ```

8. Evaluate the strength of generated passwords:
   ```
   passforge --evaluate-strength
   ```

For a full list of options, run:
```
passforge --help
```

## Configuration

PassForge allows for extensive configuration through command-line arguments. Here are the main configuration options:

- `--length` or `-l`: Set the password length (default: 18)
- `--max-length`: Set the maximum password length (for range-based generation)
- `--count` or `-c`: Number of passwords to generate (default: 1)
- `--no-capitals`: Exclude uppercase letters
- `--no-numbers`: Exclude numbers
- `--no-symbols`: Exclude symbols
- `--passphrase` or `-p`: Generate a passphrase instead of a password
- `--words` or `-w`: Number of words in the passphrase (default: 4)
- `--separator`: Separator for words in the passphrase (default: "-")
- `--word-list`: Path to a custom word list file for passphrase generation
- `--evaluate-strength` or `-e`: Show password strength evaluation
- `--preset`: Use a preset configuration (choices: Weak, Average, Strong)

## Development

PassForge is built with a modular architecture, making it easy to extend and maintain. The main components are:

- `Generator` trait: Defines the interface for password and passphrase generation
- `StrengthEvaluator` trait: Defines the interface for password strength evaluation
- `PasswordGenerator` and `PassphraseGenerator`: Implement the `Generator` trait
- `ZxcvbnAnalysis`: Implements the `StrengthEvaluator` trait using the zxcvbn algorithm

To run the tests:

```bash
cargo test
```

To run the benchmarks:

```bash
cargo bench
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [zxcvbn-rs](https://github.com/shssoichiro/zxcvbn-rs) for password strength evaluation
- [EFF's Large Wordlist](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) for passphrase generation
