use std::env;
use std::fs;

/// Chapter 12. Run code samples
pub fn run() {
    _ch_12_03_06_002();
}

// 12.1. Accepting Command Line Arguments
fn _ch_12_01() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}

// 12.2. Reading a File
fn _ch_12_02() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(&file_path)
       .expect("Something went wrong reading the file");

    println!("[12.2] With text:\n{contents}");
}

// 12.3. Refactoring to Improve Modularity and Error Handling

    // 12.3.3 - Grouping Configuration Values
    fn _ch_12_03_03() {
        struct Config {
            query: String,
            file_path: String,
        }
        
        fn parse_config(args: &[String]) -> Config {
            let query = args[1].clone();
            let file_path = args[2].clone();
            Config { query, file_path, }
        }

        let args: Vec<String> = env::args().collect();
        let config = parse_config(&args);
        println!("Searching for {}", config.query);
        println!("In file {}", config.file_path);

        let contents = fs::read_to_string(&config.file_path)
            .expect("Something went wrong reading the file");

        println!("[12.3.3] With text:\n{contents}");
    }

    // 12.3.4 - Creating a Constructor for Config
    #[allow(dead_code)]
    fn _ch_12_03_04() {
        struct Config {
            query: String,
            file_path: String,
        }

        impl Config {
            fn new(args: &[String]) -> Config {
                let query = args[1].clone();
                let file_path = args[2].clone();
                Config {
                    query, file_path,
                }
            }
        }

        let args: Vec<String> = env::args().collect();
        let config = Config::new(&args);
        println!("Searching for {}", config.query);
        println!("In file {}", config.file_path);

        let contents = fs::read_to_string(&config.file_path).expect("Something went wrong reading the file");
        println!("[12.3.4] With text:\n{contents}");
    }

    // 12.3.5 - Fixing the Error Handling

        // 12.3.5.1 - Improving the Error Message
        #[allow(dead_code)]
        fn _ch_12_03_05_001() {
            struct Config {
                query: String,
                file_path: String,
            }

            impl Config {
                fn new(args: &[String]) -> Config {
                    if args.len() < 3 {
                        panic!("Not enough arguments");
                    }

                    let query = args[1].clone();
                    let file_path = args[2].clone();
                    Config {
                        query, file_path,
                    }
                }
            }

            let args: Vec<String> = env::args().collect();
            let config = Config::new(&args);
            println!("Searching for {}", config.query);
            println!("In file {}", config.file_path);
        }

        // 12.3.5.2
        //  - Returning a Result Instead of Calling panic!
        //  - Calling Config::build and Handling Errors
        #[allow(dead_code)]
        fn _ch_12_03_05_002() {
            use std::process;

            pub struct Config {
                pub query: String,
                pub file_path: String,
            }

            impl Config {
                pub fn build(args: &[String]) -> Result<Config, &'static str> {
                    if args.len() < 3 {
                        return Err("Not enough arguments");
                    }

                    let query = args[1].clone();
                    let file_path = args[2].clone();
                    
                    Ok(Config {
                        query, file_path,
                    })
                }
            }

            let args: Vec<String> = env::args().collect();
            let config = Config::build(&args).unwrap_or_else(|err| {
                println!("Problem parsing arguments: {err}");
                process::exit(1);
            });

            println!("Searching for {}", config.query);
            println!("In file {}", config.file_path);
        }

    // 12.3.6 - Extracting Logic from main
    #[allow(dead_code)]
    fn _ch_12_03_06_001() {
        use std::process;

        pub struct Config {
            pub query: String,
            pub file_path: String,
        }

        impl Config {
            pub fn build(args: &[String]) -> Result<Config, &'static str> {
                if args.len() < 3 {
                    return Err("Not enough arguments");
                }

                let query = args[1].clone();
                let file_path = args[2].clone();
                
                Ok(Config {
                    query, file_path,
                })
            }
        }

        fn run(config: &Config) {
            let c = fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
                println!("Problem reading file: {err}");
                process::exit(1);
            });
            println!("With text:\n{}", c);
        }

        let args: Vec<String> = env::args().collect();
        let config = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });

        println!("Searching for {}", config.query);
        println!("In file {}", config.file_path);

        run(&config);
    }

    
    // Returning Errors from the run Function
    // Handling Errors Returned from run in main
    fn _ch_12_03_06_002() {
        use std::process;
        use std::error::Error;

        pub struct Config {
            pub query: String,
            pub file_path: String,
        }

        impl Config {
            pub fn build(args: &[String]) -> Result<Config, &'static str> {
                if args.len() < 3 {
                    return Err("Not enough arguments");
                }

                let query = args[1].clone();
                let file_path = args[2].clone();
                
                Ok(Config {
                    query, file_path,
                })
            }
        }

        fn run(config: &Config) -> Result<(), Box<dyn Error>> {
            let c = fs::read_to_string(&config.file_path)?;
            println!("With text:\n{}", c);
            Ok(())
        }

        let args: Vec<String> = env::args().collect();
        let config = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });

        println!("Searching for {}", config.query);
        println!("In file {}", config.file_path);

        if let Err(e) = run(&config) {
            println!("Application error: {e}");
            process::exit(1);
        }
    }
