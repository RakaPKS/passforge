fn main() {
    let custom_evaluator = MyCustomEvaluator::new();
    let config = PasswordConfig::builder()
        .length(Length::Single(20))
        .strength_evaluator(custom_evaluator)
        .build();
    let generator = PasswordGenerator;
    let password = generator.generate(&config)?;
}
