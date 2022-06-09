use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "sckrd";

#[test]
fn help_plain_short() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["-h"])
        .assert()
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn help_plain_long() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["--help"])
        .assert()
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn help_beside_short() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["-p", "-n", "8", "-h"])
        .assert()
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn help_beside_long() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["-p", "-n", "8", "--help"])
        .assert()
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn help_short_beside_version_short() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["-h", "-V"])
        .assert()
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn help_long_beside_version_short() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["--help", "-V"])
        .assert()
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn help_short_beside_version_long() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["-h", "--version"])
        .assert()
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn help_long_beside_version_long() -> TestResult {
    Command::cargo_bin(PROG)?
        .args(&["--help", "--version"])
        .assert()
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}
