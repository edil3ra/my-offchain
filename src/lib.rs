#![feature(slice_group_by)]

use serde::{Deserialize, Serialize};
use std::io::{self};
use std::{env, error::Error, fs::File};
use std::collections::{HashMap};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;


type MyResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Deserialize, Clone)]
enum TransactionTypes {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    Chargeback,
}

#[derive(Debug, Deserialize, Clone)]
struct Transaction {
    #[serde(rename = "type")]
    transaction_type: TransactionTypes,
    client: u16,
    tx: u32,
    #[serde(with = "rust_decimal::serde::float_option")]
    amount: Option<Decimal>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AccountSerialized {
    client: u16,
    #[serde(with = "rust_decimal::serde::float")]
    available: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    held: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    total: Decimal,
    locked: bool,
}

#[derive(Debug, Clone)]
struct Account {
    client: u16,
    available: Decimal,
    held: Decimal,
    locked: bool,
}

impl Account {
    fn new(client: u16) -> Self {
        Account {
            client,
            available: dec!(0),
            held: dec!(0),
            locked: false,
        }
    }

    /// could not find a way to deserialize total
    fn to_serialize(&self) -> AccountSerialized {
        AccountSerialized {
            client: self.client,
            available: self.available.round_dp(4),
            held: self.held.round_dp(4),
            total: self.total().round_dp(4),
            locked: self.locked,
        }
    }

    fn total(&self) -> Decimal {
        self.available + self.held
    }
}

pub fn run() -> MyResult<()> {
    let filename = env::args().nth(1).ok_or("Missing filename arg")?;
    let file = File::open(&filename).map_err(|e| e.to_string())?;
    let mut csv_data = csv::Reader::from_reader(&file);
    let mut transactions = csv_data
        .deserialize()
        .into_iter()
        .collect::<Result<Vec<Transaction>, csv::Error>>()?;

    transactions.sort_by_key(|transaction| transaction.client);
    let transactions_group_by_client = transactions
        .group_by(|a, b| a.client == b.client)
        .map(|group| group.to_vec())
        .collect::<Vec<Vec<Transaction>>>();

    let accounts = transactions_group_by_client
        .iter()
        .map(|transactions| {
            create_account_from_transactions(transactions).map(|account| account.to_serialize())
        })
        .collect::<Result<Vec<AccountSerialized>, Box<dyn Error>>>()?;

    let mut writer = csv::Writer::from_writer(io::stdout());
    write_to_stdout(&accounts, &mut writer)?;
    Ok(())
}

fn create_account_from_transactions(transactions: &[Transaction]) -> MyResult<Account> {
    let mut account = Account::new(transactions[0].client); // ok to panic here as there is always at least one transaction

    let deposits_map = transactions
        .iter()
        .filter(|transaction| matches!(transaction.transaction_type, TransactionTypes::Deposit))
        .map(|transaction| (transaction.tx, transaction))
        .collect::<HashMap<u32, &Transaction>>();

    let mut disputes_map: HashMap<u32, &Transaction> = HashMap::new();

    for transaction in transactions.iter() {
        match transaction.transaction_type {
            TransactionTypes::Deposit => {
                account.available += transaction.amount.unwrap();
            }
            TransactionTypes::Withdrawal => {
                if (account.available - transaction.amount.unwrap()) > dec!(0) {
                    account.available -= transaction.amount.unwrap()
                }
            }
            TransactionTypes::Dispute => {
                if let Some(deposit_transaction) = deposits_map.get(&transaction.tx) {
                    account.available -= deposit_transaction.amount.unwrap();
                    account.held += deposit_transaction.amount.unwrap();
                    disputes_map.insert(deposit_transaction.tx, deposit_transaction);
                }
            }
            TransactionTypes::Resolve => {
                if let Some((_, dispute_transaction)) = disputes_map.remove_entry(&transaction.tx) {
                    account.available += dispute_transaction.amount.unwrap();
                    account.held -= dispute_transaction.amount.unwrap();
                }
            }
            TransactionTypes::Chargeback => {
                if let Some((_, dispute_transaction)) = disputes_map.remove_entry(&transaction.tx) {
                    account.held -= dispute_transaction.amount.unwrap();
                    account.locked = true;
                }
            }
        }
    }
    Ok(account)
}

fn write_to_stdout<T: Serialize>(
    items: &[T],
    writer: &mut csv::Writer<io::Stdout>,
) -> Result<(), csv::Error> {
    for item in items {
        writer.serialize(item)?;
    }
    writer.flush()?;
    Ok(())
}
