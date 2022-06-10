use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "sckrd";

// testfiles
const TEST_SMALL: &str = "tests/inputs/test_small.bin";

#[test]
fn byte_count_100() -> TestResult {
    let output = "";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "100", "-k", "128"])
        .assert()
        .stdout(predicate::str::contains(output));

    Ok(())
}

#[test]
fn byte_count_200() -> TestResult {
    let output = "4a36662f4867425a7533496264627457: 3.875";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "200", "-k", "128"])
        .assert()
        .stdout(predicate::str::contains(output));

    Ok(())
}

#[test]
fn byte_count_500() -> TestResult {
    let output_00 = "4a36662f4867425a7533496264627457: 3.875";
    let output_01 = "344e456141462f4a76563970576a646f: 4";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "500", "-k", "128"])
        .assert()
        .stdout(predicate::str::contains(output_00))
        .stdout(predicate::str::contains(output_01));

    Ok(())
}
