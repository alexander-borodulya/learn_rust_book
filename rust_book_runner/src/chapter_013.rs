use std::env;
use std::process;

use chapter_013::Config;

/// Chapter 13.3 - Improving Our I/O Project
pub fn run(_subchapter_index: u32) {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = chapter_013::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
