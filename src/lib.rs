#![feature(slice_group_by)]

use serde::{Deserialize, Serialize};
use std::io::{self};
use std::{env, error::Error, fmt, fs::File};
use std::collections::{HashSet, HashMap};

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
    amount: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AccountSerialized {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

#[derive(Debug, Clone)]
struct Account {
    client: u16,
    available: f32,
    held: f32,
    locked: bool,
}

impl Account {
    fn new(client: u16) -> Self {
        Account {
            client,
            available: 0.0,
            held: 0.0,
            locked: false,
        }
    }

    /// could not find a way to deserialize total
    fn to_serialize(&self) -> AccountSerialized {
        AccountSerialized {
            client: self.client,
            available: self.available,
            held: self.held,
            total: self.total(),
            locked: self.locked,
        }
    }

    fn total(&self) -> f32 {
        self.available + self.held
    }
}

pub fn run() -> MyResult<()> {
    let filename = env::args().nth(1).ok_or("missing filename")?;
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

    // dbg!(&file);
    // dbg!(&csv_data);
    // dbg!(&transactions);
    // dbg!(&transactions_group_by_client);
    // dbg!(accounts);

    Ok(())
}

fn create_account_from_transactions(transactions: &[Transaction]) -> MyResult<Account> {
    let mut account = Account::new(transactions[0].client); // ok to panic here as there is always at least one transaction
    let deposits = transactions
        .iter()
        .filter(|transaction| matches!(transaction.transaction_type, TransactionTypes::Deposit));

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
                if (account.available - transaction.amount.unwrap()) > 0.0 {
                    account.available -= transaction.amount.unwrap()
                }
            }
            TransactionTypes::Dispute => {
                if let Some(deposit_transaction) = deposits_map.get(&transaction.tx) {
                    account.available -= deposit_transaction.amount.unwrap();
                    account.held += deposit_transaction.amount.unwrap();
                    disputes_map.insert(transaction.tx, transaction);
                }
            }
            TransactionTypes::Resolve => {
                let transaction = deposits
                    .clone()
                    .find(|deposit| deposit.tx == transaction.tx);
                if let Some(element) = transaction {
                    account.available += element.amount.unwrap();
                    account.held -= element.amount.unwrap();
                }
            }
            TransactionTypes::Chargeback => todo!(),
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
