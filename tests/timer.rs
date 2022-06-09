use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "sckrd";

// testfiles
const TEST_SMALL: &str = "tests/inputs/test_small.bin";

#[test]
fn show_elapsed_time() -> TestResult {
    let output = "Program executed in";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-t"])
        .assert()
        .stdout(predicate::str::contains(output));

    Ok(())
}
