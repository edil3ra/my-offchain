use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

struct Test {
    input: &'static str,
    out: &'static str,
}
const PRG: &str = "my-offchain";

const WITHDRAWAL: Test = Test {
    input: "tests/inputs/in_withdrawal.csv",
    out: "tests/expected/out_withdrawal.csv",
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


#[test]
fn withdrawal_sucess() -> TestResult {
    run(&WITHDRAWAL)
}
