use clap::Parser;
use rust_book_runner::{cli_args::Args, runner::Runner};

fn main() {
    if let Err(e) = Runner::run(Args::parse()) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
