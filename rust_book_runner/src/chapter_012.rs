use std::env;
use std::process;

use chapter_012::Config;

/// Chapter 12. An I/O Project: Building a Command Line Program
pub fn run(_subchapter_index: u32) {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    if let Err(e) = chapter_012::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
