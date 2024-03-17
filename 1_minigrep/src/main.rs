use std::process;
use minigrep::Arguments;
use clap::Parser;

fn main() {
    let args = Arguments::parse();

    if let Err(e) = minigrep::run(args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}