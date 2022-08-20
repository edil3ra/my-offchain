use std::{env, fs::File};

fn main() {
    let filename = env::args().nth(1).expect("panic!");
    let file = File::open(&filename).unwrap();
    dbg!(filename);
    dbg!(file);
}
