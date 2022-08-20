use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;


const PRG: &str = "my-offchain";

type TestResult = Result<(), Box<dyn std::error::Error>>;

struct TestSuccess {
    input: &'static str,
    out: &'static str,
}

struct TestFailure {
    input: &'static str,
}

const WITHDRAWAL_SUCESS: TestSuccess = TestSuccess {
    input: "tests/inputs/in_withdrawal_success.csv",
    out: "tests/expected/out_withdrawal_success.csv",
};


const WITHDRAWAL_FAILURE: TestFailure = TestFailure {
    input: "tests/inputs/in_withdrawal_failure.csv",
};


fn run(test: &TestSuccess) -> TestResult {
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
fn withdrawal_sucess() -> TestResult {
    run(&WITHDRAWAL_SUCESS)
}


#[test]
fn withdrawal_should_fail_when_cannot_withdrawal() -> TestResult {
    run_fail(&WITHDRAWAL_FAILURE, "error")
}
