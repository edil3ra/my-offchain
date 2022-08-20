use std::{env, fs::File, error::Error};


pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = env::args().nth(1).ok_or("missing filename")?;
    let file = File::open(&filename).map_err(|e| e.to_string())?;
    dbg!(filename);
    dbg!(file);
    Ok(())
}
