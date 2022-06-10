use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "sckrd";

// testfiles
const TEST_SMALL: &str = "tests/inputs/test_small.bin";

#[test]
fn show_basic_output() -> TestResult {
    let output = "4943445771524d4d55646648344e4561";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-p", "-k", "128"])
        .assert()
        .stdout(predicate::str::contains(output));

    Ok(())
}
