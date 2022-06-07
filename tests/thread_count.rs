use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "sckrd";

// testfiles
const TEST_SMALL: &str = "tests/inputs/test_small.bin";

#[test]
fn thread_count_default() -> TestResult {
    let output_00 = "lHGJ6f/HgBZu3IbdbtWYhE6cm77TeUWx: 4.6875";
    let output_01 = "ICDWqRMMUdfH4NEaAF/JvV9pWjdoOzKp: 4.75";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "500"])
        .assert()
        .stdout(predicate::str::contains(output_00))
        .stdout(predicate::str::contains(output_01));

    Ok(())
}

#[test]
fn thread_count_1() -> TestResult {
    let output_00 = "lHGJ6f/HgBZu3IbdbtWYhE6cm77TeUWx: 4.6875";
    let output_01 = "ICDWqRMMUdfH4NEaAF/JvV9pWjdoOzKp: 4.75";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "500", "-n", "1"])
        .assert()
        .stdout(predicate::str::contains(output_00))
        .stdout(predicate::str::contains(output_01));

    Ok(())
}

#[test]
fn thread_count_2() -> TestResult {
    let output_00 = "lHGJ6f/HgBZu3IbdbtWYhE6cm77TeUWx: 4.6875";
    let output_01 = "ICDWqRMMUdfH4NEaAF/JvV9pWjdoOzKp: 4.75";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "500", "-n", "2"])
        .assert()
        .stdout(predicate::str::contains(output_00))
        .stdout(predicate::str::contains(output_01));

    Ok(())
}

#[test]
fn thread_count_8() -> TestResult {
    let output_00 = "lHGJ6f/HgBZu3IbdbtWYhE6cm77TeUWx: 4.6875";
    let output_01 = "ICDWqRMMUdfH4NEaAF/JvV9pWjdoOzKp: 4.75";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "500", "-n", "8"])
        .assert()
        .stdout(predicate::str::contains(output_00))
        .stdout(predicate::str::contains(output_01));

    Ok(())
}


#[test]
fn thread_count_16() -> TestResult {
    let output_00 = "lHGJ6f/HgBZu3IbdbtWYhE6cm77TeUWx: 4.6875";
    let output_01 = "ICDWqRMMUdfH4NEaAF/JvV9pWjdoOzKp: 4.75";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "500", "-n", "16"])
        .assert()
        .stdout(predicate::str::contains(output_00))
        .stdout(predicate::str::contains(output_01));

    Ok(())
}

#[test]
fn thread_count_32() -> TestResult {
    let output_00 = "lHGJ6f/HgBZu3IbdbtWYhE6cm77TeUWx: 4.6875";
    let output_01 = "ICDWqRMMUdfH4NEaAF/JvV9pWjdoOzKp: 4.75";

    Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "500", "-n", "32"])
        .assert()
        .stdout(predicate::str::contains(output_00))
        .stdout(predicate::str::contains(output_01));

    Ok(())
}

#[test]
fn thread_count_range() -> TestResult {
    let output_00 = "lHGJ6f/HgBZu3IbdbtWYhE6cm77TeUWx: 4.6875";
    let output_01 = "ICDWqRMMUdfH4NEaAF/JvV9pWjdoOzKp: 4.75";

    for i in 0..32 {
        Command::cargo_bin(PROG)?
        .args(&["-i", TEST_SMALL, "-b", "500", "-n", &(i.to_string())])
        .assert()
        .stdout(predicate::str::contains(output_00))
        .stdout(predicate::str::contains(output_01));
    }
    
    Ok(())
}