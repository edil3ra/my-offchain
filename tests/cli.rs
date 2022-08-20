use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

const PRG: &str = "my-offchain";

type TestResult = Result<(), Box<dyn std::error::Error>>;

struct Test {
    input: &'static str,
    out: &'static str,
}

struct TestFailure {
    input: &'static str,
}

const WITHDRAWAL_SUCESS: Test = Test {
    input: "tests/inputs/in_withdrawal_success.csv",
    out: "tests/expected/out_withdrawal_success.csv",
};

const WITHDRAWAL_IGNORED: Test = Test {
    input: "tests/inputs/in_withdrawal_ignored.csv",
    out: "tests/expected/out_withdrawal_ignored.csv",
};

const DISPUTE_EXISTING_FUND: Test = Test {
    input: "tests/inputs/in_dispute_existing.csv",
    out: "tests/expected/out_dispute_existing.csv",
};


fn run(test: &Test) -> TestResult {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(test.out)?;

    Command::cargo_bin(PRG)?
        .arg(test.input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run_fail(test: &TestFailure, message: &str) -> TestResult {
    let input = fs::read_to_string(test.input)?;

    Command::cargo_bin(PRG)?
        .arg(test.input)
        .assert()
        .success()
        .stderr(predicate::str::contains(message));
    Ok(())
}

#[test]
fn should_withdrawal_when_funds_are_available() -> TestResult {
    run(&WITHDRAWAL_SUCESS)
}

#[test]
fn should_not_withdrawal_when_funds_is_not_available() -> TestResult {
    run(&WITHDRAWAL_IGNORED)
}

#[test]
fn should_held_funds_when_dispute_on_existing_tx() -> TestResult {
    run(&DISPUTE_EXISTING_FUND)
}
