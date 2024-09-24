use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_default_password_generation() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.assert().success().stdout(
        predicate::str::is_match(r"^[a-zA-Z0-9!@#$%^&*()-_=+\[\]{}|;:,.<>?]{18}\n$").unwrap(),
    );
}

#[test]
fn test_cli_custom_length_password() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.arg("--length").arg("24").assert().success().stdout(
        predicate::str::is_match(r"^[a-zA-Z0-9!@#$%^&*()-_=+\[\]{}|;:,.<>?]{24}\n$").unwrap(),
    );
}

#[test]
fn test_cli_no_symbols_password() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.arg("--no-symbols")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[a-zA-Z0-9]{18}\n$").unwrap());
}

#[test]
fn test_cli_passphrase_generation() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.arg("--passphrase")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^(\w+(-\w+){5})\n$").unwrap());
}

#[test]
fn test_cli_multiple_passwords() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.arg("--count").arg("3").assert().success().stdout(
        predicate::str::is_match(r"^([a-zA-Z0-9!@#$%^&*()-_=+\[\]{}|;:,.<>?]{18}\n){3}$").unwrap(),
    );
}

#[test]
fn test_cli_invalid_length() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.arg("--length")
        .arg("0")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error: Invalid password length"));
}

#[test]
fn test_cli_strength_evaluation() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.arg("--evaluate-strength")
        .assert()
        .success()
        .stdout(predicate::str::contains("Strength: Score:"))
        .stdout(predicate::str::contains("Crack time:"));
}

#[test]
fn test_cli_no_capitals() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.arg("--no-capitals")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[a-z0-9!@#$%^&*()-_=+\[\]{}|;:,.<>?]{18}\n$").unwrap());
}

#[test]
fn test_cli_custom_passphrase() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.args(["--passphrase", "--words", "5", "--separator", "_"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^(\w+(_\w+){4})\n$").unwrap());
}

#[test]
fn test_cli_password_length_range() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.args(["--min-length", "10", "--max-length", "20"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(r"^[a-zA-Z0-9!@#$%^&*()-_=+\[\]{}|;:,.<>?]{10,20}\n$")
                .unwrap(),
        );
}

#[test]
fn test_cli_invalid_word_list() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.args(["--passphrase", "--word-list", "nonexistent_file.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_cli_multiple_items_format() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.args(["--count", "3"]).assert().success().stdout(
        predicate::str::is_match(r"^([a-zA-Z0-9!@#$%^&*()-_=+\[\]{}|;:,.<>?]{18}\n){3}$").unwrap(),
    );
}

#[test]
fn test_cli_strength_evaluation_format() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.arg("--evaluate-strength")
        .assert()
        .success()
        .stdout(predicate::str::contains("Score:"))
        .stdout(predicate::str::contains("Crack time:"));
}

#[test]
fn test_cli_extreme_length() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.args(["--length", "1000"]).assert().success().stdout(
        predicate::str::is_match(r"^[a-zA-Z0-9!@#$%^&*()-_=+\[\]{}|;:,.<>?]{1000}\n$").unwrap(),
    );
}
