use std::env;
use std::process;

use learn_rust_book::chapter_012_lib::Config;

pub fn run() {
    println!("12. An I/O Project: Building a Command Line Program");
    run_main();
}

fn run_main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = learn_rust_book::chapter_012_lib::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
