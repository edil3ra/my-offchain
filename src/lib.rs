#![feature(slice_group_by)]

use serde::{Deserialize, Serialize};
use std::io::{self};
use std::{env, error::Error, fs::File};

#[derive(Debug, Deserialize, Clone)]
struct Transaction {
    #[serde(rename = "type")]
    transaction_type: String,
    client: u16,
    tx: u32,
    amount: Option<f32>,
}

#[derive(Debug, Clone, Serialize)]
struct Account {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

impl Account {
    fn new(client: u16) -> Self {
        Account {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
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

    let accounts = vec![Account::new(1), Account::new(2)];
    let mut writer = csv::Writer::from_writer(io::stdout());
    write_to_stdout(&accounts, &mut writer)?;

    // dbg!(&file);
    // dbg!(&csv_data);
    // dbg!(&transactions);
    // dbg!(&transactions_group_by_client);
    // dbg!(accounts);

    Ok(())
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
