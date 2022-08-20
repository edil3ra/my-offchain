use assert_cmd::Command;
use std::fs;

const PRG: &str = "my-offchain";

type TestResult = Result<(), Box<dyn std::error::Error>>;

struct Test {
    input: &'static str,
    out: &'static str,
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

const DISPUTE_NON_EXISTING_FUND: Test = Test {
    input: "tests/inputs/in_dispute_non_existing.csv",
    out: "tests/expected/out_dispute_non_existing.csv",
};

const RESOLVE_FUND: Test = Test {
    input: "tests/inputs/in_resolve.csv",
    out: "tests/expected/out_resolve.csv",
};

const RESOLVE_NOT_DISPUTED_FUND: Test = Test {
    input: "tests/inputs/in_resolve_not_disputed.csv",
    out: "tests/expected/out_resolve_not_disputed.csv",
};

const RESOLVE_CHARGEBACK: Test = Test {
    input: "tests/inputs/in_chargeback.csv",
    out: "tests/expected/out_chargeback.csv",
};

const RESOLVE_NOT_DISPUTED_CHARGEBACK: Test = Test {
    input: "tests/inputs/in_chargeback_not_disputed.csv",
    out: "tests/expected/out_chargeback_not_disputed.csv",
};

const DEPOSIT_MULTIPLE_CLIENT: Test = Test {
    input: "tests/inputs/in_deposit_with_multiple_clients.csv",
    out: "tests/expected/out_deposit_with_multiple_clients.csv",
};



fn run(test: &Test) -> TestResult {
    let expected = fs::read_to_string(test.out)?;

    Command::cargo_bin(PRG)?
        .arg(test.input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn should_withdraw_when_funds_are_available() -> TestResult {
    run(&WITHDRAWAL_SUCESS)
}

#[test]
fn should_not_withdraw_when_funds_is_not_available() -> TestResult {
    run(&WITHDRAWAL_IGNORED)
}

#[test]
fn should_held_funds_when_dispute_has_an_existing_deposit() -> TestResult {
    run(&DISPUTE_EXISTING_FUND)
}

#[test]
fn should_not_held_funds_when_dispute_missing_a_deposit() -> TestResult {
    run(&DISPUTE_NON_EXISTING_FUND)
}

#[test]
fn should_resolve_funds_when_dispute_exist() -> TestResult {
    run(&RESOLVE_FUND)
}

#[test]
fn should_not_resolve_funds_when_dispute_does_not_exist() -> TestResult {
    run(&RESOLVE_NOT_DISPUTED_FUND)
}

#[test]
fn should_withdraw_and_freeze_account_on_chargeback() -> TestResult {
    run(&RESOLVE_CHARGEBACK)
}

#[test]
fn should_not_withdraw_funds_when_dispute_does_not_exist() -> TestResult {
    run(&RESOLVE_NOT_DISPUTED_CHARGEBACK)
}

#[test]
fn should_work_with_mutiple_clients() -> TestResult {
    run(&DEPOSIT_MULTIPLE_CLIENT)
}
