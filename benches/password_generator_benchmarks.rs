use criterion::{black_box, criterion_group, criterion_main, Criterion};
use passforge::{
    Generator, Length, PassphraseConfig, PassphraseGenerator, PasswordConfig, PasswordGenerator,
    StrengthEvaluator, WordList, ZxcvbnAnalysis,
};

fn generate_passwords(c: &mut Criterion) {
    let config = PasswordConfig::new(Length::Single(16), true, true, true);
    c.bench_function("generate 1000 16-char passwords", |b| {
        b.iter(|| {
            black_box(PasswordGenerator::generate_multiple(&config, 1000).unwrap());
        })
    });
}

fn generate_passphrases(c: &mut Criterion) {
    let config = PassphraseConfig::new(5, "-".to_string(), WordList::Default);
    c.bench_function("generate 100 5-word passphrases", |b| {
        b.iter(|| {
            black_box(PassphraseGenerator::generate_multiple(&config, 100).unwrap());
        })
    });
}

fn evaluate_password_strength(c: &mut Criterion) {
    let config = PasswordConfig::new(Length::Single(16), true, true, true);
    let passwords = PasswordGenerator::generate_multiple(&config, 100).unwrap();
    c.bench_function("evaluate strength of 100 passwords", |b| {
        b.iter(|| {
            for password in &passwords {
                black_box(ZxcvbnAnalysis::evaluate(password).unwrap());
            }
        })
    });
}

criterion_group!(
    benches,
    generate_passwords,
    generate_passphrases,
    evaluate_password_strength
);
criterion_main!(benches);
