use serde::Deserialize;
use std::{env, error::Error, fs::File};

#[derive(Debug, Deserialize, Clone)]
struct Transaction {
    #[serde(rename = "type")]
    transaction_type: String,
    client: u16,
    txt: u32,
    amount: Option<f32>,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = env::args().nth(1).ok_or("missing filename")?;
    let file = File::open(&filename).map_err(|e| e.to_string())?;
    let mut csv_data = csv::Reader::from_reader(&file);
    let transactions = csv_data
        .deserialize()
        .into_iter()
        .collect::<Result<Vec<Transaction>, csv::Error>>()?;

    dbg!(&file);
    dbg!(&csv_data);
    dbg!(&transactions);
    Ok(())
}
