use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "sckrd";

#[test]
fn version_plain_short() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["-V"])
        .assert()
        .stdout(predicate::str::contains("sckrd 0.1.0"));

    Ok(())
}

#[test]
fn version_plain_long() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["--version"])
        .assert()
        .stdout(predicate::str::contains("sckrd 0.1.0"));

    Ok(())
}

#[test]
fn version_beside_short() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["-p", "-n", "8", "-V"])
        .assert()
        .stdout(predicate::str::contains("sckrd 0.1.0"));

    Ok(())
}

#[test]
fn version_beside_long() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["-p", "-n", "8", "--version"])
        .assert()
        .stdout(predicate::str::contains("sckrd 0.1.0"));

    Ok(())
}

#[test]
fn version_short_beside_help_short() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["-V", "-h"])
        .assert()
        .stdout(predicate::str::contains("sckrd 0.1.0"));

    Ok(())
}

#[test]
fn version_long_beside_help_short() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["--version", "-h"])
        .assert()
        .stdout(predicate::str::contains("sckrd 0.1.0"));

    Ok(())
}

#[test]
fn version_short_beside_help_long() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["-V", "--help"])
        .assert()
        .stdout(predicate::str::contains("sckrd 0.1.0"));

    Ok(())
}

#[test]
fn version_long_beside_help_long() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["--version", "--help"])
        .assert()
        .stdout(predicate::str::contains("sckrd 0.1.0"));

    Ok(())
}
