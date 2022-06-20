use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "sckrd";

// testfiles
// const TEST_SMALL: &str = "tests/inputs/test_small.bin";
// const TEST_MIDDLE: &str = "tests/inputs/test_middle.bin";
// const TEST_LARGE: &str = "tests/inputs/test_large.bin";

#[test]
fn dies_verbose_and_basic_output() -> TestResult {
    let msg = "The argument '--verbose' cannot be used with '--basic-output'";

    Command::cargo_bin(PROG)?
        .args(&["-v", "-p"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

#[test]
fn dies_minimal_and_verbose_output() -> TestResult {
    let msg = "The argument '--basic-output' cannot be used with '--verbose'";

    Command::cargo_bin(PROG)?
        .args(&["-p", "-v"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

#[test]
fn dies_suppress_and_verbose_output() -> TestResult {
    let msg = "The argument '--suppress-output' cannot be used with '--verbose'";

    Command::cargo_bin(PROG)?
        .args(&["-s", "-v"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

#[test]
fn dies_suppress_and_minimal_output() -> TestResult {
    let msg = "The argument '--suppress-output' cannot be used with '--basic-output'";

    Command::cargo_bin(PROG)?
        .args(&["-s", "-p"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}
