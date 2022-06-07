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
        .args(&["-i", TEST_SMALL, "-b", "100"])
        .assert()
        .stdout(predicate::str::contains(output));

    Ok(())
}

#[test]
fn byte_count_200() -> TestResult {
    let output = "lHGJ6f/HgBZu3IbdbtWYhE6cm77TeUWx: 4.6875";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "200"])
        .assert()
        .stdout(predicate::str::contains(output));

    Ok(())
}

#[test]
fn byte_count_500() -> TestResult {
    let output_00 = "lHGJ6f/HgBZu3IbdbtWYhE6cm77TeUWx: 4.6875";
    let output_01 = "ICDWqRMMUdfH4NEaAF/JvV9pWjdoOzKp: 4.75";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "500"])
        .assert()
        .stdout(predicate::str::contains(output_00))
        .stdout(predicate::str::contains(output_01));

    Ok(())
}