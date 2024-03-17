use std::process;
use minigrep::Config;
use clap::Parser;

fn main() {
    let config = Config::parse();

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}