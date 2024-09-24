use criterion::{black_box, criterion_group, criterion_main, Criterion};
use passgen::{Generator, PasswordConfig, PasswordGenerator};
use rayon::prelude::*;

fn sequential_generation(config: &PasswordConfig, count: usize) -> Vec<String> {
    (0..count)
        .map(|_| PasswordGenerator::generate(config).unwrap())
        .collect()
}

fn parallel_generation(config: &PasswordConfig, count: usize) -> Vec<String> {
    (0..count)
        .into_par_iter()
        .map(|_| PasswordGenerator::generate(config).unwrap())
        .collect()
}

fn single_vs_multi_threaded_password_gen(c: &mut Criterion) {
    let config = PasswordConfig::builder()
        .length(passgen::Length::Single(16))
        .build();

    let mut group = c.benchmark_group("Password Generation");
    for count in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(format!("Sequential {}", count), count, |b, &count| {
            b.iter(|| sequential_generation(black_box(&config), black_box(count)))
        });
        group.bench_with_input(format!("Parallel {}", count), count, |b, &count| {
            b.iter(|| parallel_generation(black_box(&config), black_box(count)))
        });
    }
    group.finish();
}

criterion_group!(benches, single_vs_multi_threaded_password_gen);
criterion_main!(benches);
