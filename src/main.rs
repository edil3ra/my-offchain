fn main() {
    if let Err(e) = my_offchain::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
