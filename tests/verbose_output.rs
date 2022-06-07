use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "sckrd";

// testfiles
const TEST_SMALL: &str = "tests/inputs/test_small.bin";

#[test]
fn show_verbose_output() -> TestResult {
    let output_00 = "Possible Key:";
    let output_01 = "Entropy:";
    let output_02 = "Size (Byte):";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-v"])
        .assert()
        .stdout(predicate::str::contains(output_00))
        .stdout(predicate::str::contains(output_01))
        .stdout(predicate::str::contains(output_02));

    Ok(())
}