use clap::Parser;
use rust_book_runner::{runner::Runner, cli_args::Args};

fn main() {
    if let Err(e) = Runner::run(Args::parse()) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
