use passforge::{
    Generator, Length, PassphraseConfig, PassphraseGenerator, PasswordConfig, PasswordGenerator,
    StrengthEvaluator, WordList, ZxcvbnAnalysis,
};

#[test]
fn test_password_generation_api() {
    let config = PasswordConfig::new(Length::Single(16), true, true, true);

    let password = PasswordGenerator::generate(&config).expect("Failed to generate password");

    assert_eq!(password.len(), 16);
}

#[test]
fn test_password_generation_no_symbols() {
    let config = PasswordConfig::new(Length::Single(16), true, true, false);

    let password = PasswordGenerator::generate(&config).expect("Failed to generate password");

    assert_eq!(password.len(), 16);
    assert!(!password
        .chars()
        .any(|c| "!@#$%^&*()-_=+[]{}|;:,.<>?".contains(c)));
}

#[test]
fn test_password_generation_range_length() {
    let config = PasswordConfig::new(Length::Range(10..=20), true, true, true);

    let password = PasswordGenerator::generate(&config).expect("Failed to generate password");

    assert!(password.len() >= 10 && password.len() <= 20);
}

#[test]
fn test_multiple_password_generation() {
    let config = PasswordConfig::new(Length::Single(16), true, true, true);

    let passwords =
        PasswordGenerator::generate_multiple(&config, 5).expect("Failed to generate passwords");

    assert_eq!(passwords.len(), 5);
    for password in passwords {
        assert_eq!(password.len(), 16);
    }
}

#[test]
fn test_passphrase_generation_api() {
    let config = PassphraseConfig::new(4, "-".to_string(), WordList::Default);

    let passphrase = PassphraseGenerator::generate(&config).expect("Failed to generate passphrase");

    let words: Vec<&str> = passphrase.split('-').collect();
    assert_eq!(words.len(), 4);
}

#[test]
fn test_passphrase_custom_separator() {
    let config = PassphraseConfig::new(4, "_".to_string(), WordList::Default);

    let passphrase = PassphraseGenerator::generate(&config).expect("Failed to generate passphrase");

    let words: Vec<&str> = passphrase.split('_').collect();
    assert_eq!(words.len(), 4);
}

#[test]
fn test_strength_evaluation() {
    let password = "w".to_string();
    let evaluation =
        ZxcvbnAnalysis::evaluate(&password).expect("Failed to evaluate password strength");
    assert!(evaluation.contains("Score: 0/4"));

    let password = "StrongP@ssw0rd!AreAmazing!@#!$!".to_string();
    let evaluation =
        ZxcvbnAnalysis::evaluate(&password).expect("Failed to evaluate password strength");
    assert!(evaluation.contains("Score: 4/4"));
}

#[test]
fn test_strength_threshold() {
    let weak_password = "weak".to_string();
    assert!(!ZxcvbnAnalysis::passes_threshold(&weak_password).expect("Failed to check threshold"));

    let strong_password = "StrongP@ssw0rd!".to_string();
    assert!(ZxcvbnAnalysis::passes_threshold(&strong_password).expect("Failed to check threshold"));
}
