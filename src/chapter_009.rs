use std::error::Error;

pub fn run() {
    println!("Chapter 9: Error Handling");
    chapter_009_1();
    run_chapter_009_2().unwrap();
    chapter_009_3();
}

fn chapter_009_3() {
    println!("Chapter 9.3: To panic! or Not to panic!");
    // Creating Custom Types for Validation
    {
        // Check bounds using if...
        let secret_number = 100;
        loop {
            let guess_str = "100";

            let guess: i32 = match guess_str.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid guess_str: {}", guess_str);
                    continue;
                }
            };

            if guess < 1 || guess > 100 {
                println!("The secret number must be between 1 and 100.");
                continue;
            }

            match guess.cmp(&secret_number) {
                std::cmp::Ordering::Less => println!("Less: guess = {}, secret_number = {}", guess, secret_number),
                std::cmp::Ordering::Equal => {
                    println!("Equal: guess = {}, secret_number = {}", guess, secret_number);
                    break;
                }
                std::cmp::Ordering::Greater => println!("Greater: guess = {}, secret_number = {}", guess, secret_number),
            }
        }

        // Create special types that might accept certain range of values.
        {
            #[derive(Debug)]
            pub struct Guess {
                value: i32,
            }

            impl Guess {
                pub fn new(value: i32) -> Guess {
                    if value < 1 || value > 100 {
                        panic!("Guess range must be between 1 and 100: {}", value);
                    }
                    Guess { value }
                }

                pub fn value(&self) -> i32 { 
                    self.value
                }
            }

            let g1 = Guess::new(99);
            println!("g1: {:?}", &g1);
            println!("g1.value: {}", g1.value());

            //
            // Calling this...
            //
            //let g2 = Guess::new(101);
            //println!("g2: {:?}", &g2);
            //
            // Produces the following output:
            //
            // Chapter 9: Error Handling
            // Chapter 9.3: To panic! or Not to panic!
            // Equal: guess = 100, secret_number = 100
            // thread 'main' panicked at 'Guess range must be between 1 and 100: 101', src\chapter_009.rs:52:25
            // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
            // error: process didn't exit successfully: `target\debug\learn-rust-book.exe` (exit code: 101)
        }
    }
}

#[allow(dead_code)]
pub fn run_chapter_009_2() -> Result<(), Box<dyn Error>> {
    println!("Chapter 9.2: Recoverable Errors with Result");

    // Basic Result with match
    {
        use std::fs::File;
        use std::io::ErrorKind;

        let f_path = "src/chapter_009_2.txt";

        let f = File::open(f_path);
        println!("f: {:?}", f);

        let _f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(f_path) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Error creating the file {f_path}: {e}"),
                },
                other_error => {
                    panic!("Problem opening the file {f_path}: {other_error}")
                }
            },
        };
    }

    // Rusult with closures and the unwrap_or_else method
    {
        use std::fs::File;
        use std::io::ErrorKind;

        let f_path = "src/chapter_009_2.txt";

        let _f = File::open(f_path).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(f_path).unwrap_or_else(|error| {
                    panic!("Error creating the file {f_path}: {error}");
                })
            } else {
                panic!("Problem opening the file {f_path}: {error}");
            }
        });

        std::fs::remove_file(f_path).unwrap_or_else(|error| {
            println!("Error removing the file {f_path}: {error}");
        });
    }

    // Shortcuts for Panic on Error: unwrap and expect
    {
        // 1
        //
        // use std::fs::File;
        // let f_path = "src/chapter_009_2.txt";
        // let _f = File::open(f_path).unwrap();
        //
        // ... produces the following output:
        //
        // Chapter 9: Error Handling
        // Chapter 9.2: Recoverable Errors with Result
        // f: Err(Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." })
        // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }', src\chapter_009.rs:62:36
        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        // error: process didn't exit successfully: `target\debug\learn-rust-book.exe` (exit code: 101)

        // 2
        //
        // use std::fs::File;
        // let f_path = "src/chapter_009_2.txt";
        // let _f = File::open(f_path).expect("<<< CUSTOM ERROR MESSAGE >>> Failed to open file [src/chapter_009_2.txt]");
        //
        //... produces the following output:
        //
        // Chapter 9: Error Handling
        // Chapter 9.2: Recoverable Errors with Result
        // f: Err(Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." })
        // thread 'main' panicked at '<<< CUSTOM ERROR MESSAGE >>> Failed to open file [src/chapter_009_2.txt]: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified."
        // }', src\chapter_009.rs:77:37
        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        // error: process didn't exit successfully: `target\debug\learn-rust-book.exe` (exit code: 101)
    }

    // Propagating Errors
    {
        use std::fs::File;
        use std::io::{Error, Read};

        fn read_username_from_file(f_path: &str) -> Result<String, Error> {
            let f = File::open(f_path);

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();

            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }

        let f_path = "src/main.rs";
        let read_result = read_username_from_file(f_path);
        match read_result {
            Ok(user_name_value) => println!("read_username_from_file: Ok: {:?}", user_name_value),
            Err(read_error_value) => {
                println!("read_username_from_file: Err: {:?}", read_error_value)
            }
        }
    }

    // A Shortcut for Propagating Errors: the ? Operator
    {
        use std::fs;
        use std::fs::File;
        use std::io;
        use std::io::Read;

        let f_path = "src/main.rs_not_found";

        fn read_username_from_file(f_path: &str) -> Result<String, io::Error> {
            let mut f = File::open(f_path)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }

        fn read_username_from_file_shorter(f_path: &str) -> Result<String, io::Error> {
            let mut s = String::new();
            File::open(f_path)?.read_to_string(&mut s)?;
            Ok(s)
        }

        fn read_username_from_file_std(f_path: &str) -> Result<String, io::Error> {
            fs::read_to_string(f_path)
        }

        let read_result = read_username_from_file(f_path);
        println!("read_username_from_file: {:?}", read_result);

        let read_result_shorter = read_username_from_file_shorter(f_path);
        println!("read_username_from_file_shorter: {:?}", read_result_shorter);

        let read_result_std = read_username_from_file_std(f_path);
        println!("read_username_from_file_std: {:?}", read_result_std);
    }

    // Where The ? Operator Can Be Used
    {
        use std::fs::File;

        fn last_char_of_first_line(text: &str) -> Option<char> {
            text.lines().next()?.chars().last()
        }

        let a = last_char_of_first_line("a b c d e f g h i j k l m n o p q r s t u v v w x y z");
        let b = last_char_of_first_line("123_456");
        let c = last_char_of_first_line("");
        println!("a: {:?}", a);
        println!("b: {:?}", b);
        println!("c: {:?}", c);

        // Calling...
        //
        // File::open("test").expect("ERROR MESSAGE: EXPECT TO BE PRINTED IN THE OUTPUT");
        //File::open("test")?; // Propagating error without printing additional information from the user
        //
        // ..produces the following output in the terminal:
        //
        // error: process didn't exit successfully: `target\debug\learn-rust-book.exe` (exit code: 101)
    }
    Ok(())
}

#[allow(dead_code)]
fn chapter_009_1() {
    println!("Chapter 9.1: Unrecoverable errors with panic");

    // 1
    // The call...
    // panic!("call panic");
    //
    // ... produces the following output:
    //
    // thread 'main' panicked at 'call panic', src\chapter_009.rs:8:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\learn-rust-book.exe` (exit code: 101)

    // 2
    // When running with...
    // RUST_BACKTRACE=1 cargo run
    // The call...
    //
    // panic!("call panic");
    //
    // ... produces the following output:
    //
    // Chapter 9: Error Handling
    // Chapter 9.1: Unrecoverable errors with panic
    // thread 'main' panicked at 'call panic', src\chapter_009.rs:21:5
    // stack backtrace:
    //    0: std::panicking::begin_panic_handler
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library\std\src\panicking.rs:584
    //    1: core::panicking::panic_fmt
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library\core\src\panicking.rs:142
    //    2: learn_rust_book::chapter_009::chapter_009_1
    //              at .\src\chapter_009.rs:21
    //    3: learn_rust_book::chapter_009::run
    //              at .\src\chapter_009.rs:3
    //    4: learn_rust_book::main
    //              at .\src\main.rs:21
    //    5: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc\library\core\src\ops\function.rs:248
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    // error: process didn't exit successfully: `target\debug\learn-rust-book.exe` (exit code: 101)

    // 3
    // When running with...
    // cargo run
    //
    // let v = vec![1, 2, 3];
    // let _v_100 = v[100];
    //
    // ... produces the following output:
    //
    // Chapter 9: Error Handling
    // Chapter 9.1: Unrecoverable errors with panic
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 100', src\chapter_009.rs:44:18
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\learn-rust-book.exe` (exit code: 101)

    // 4
    // When running with...
    // RUST_BACKTRACE=1 cargo run
    //
    // let v = vec![1, 2, 3];
    // let _v_100 = v[100];
    //
    // ... produces the following output:
    //
    // Chapter 9: Error Handling
    // Chapter 9.1: Unrecoverable errors with panic
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 100', src\chapter_009.rs:61:18
    // stack backtrace:
    //    0: std::panicking::begin_panic_handler
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library\std\src\panicking.rs:584
    //    1: core::panicking::panic_fmt
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library\core\src\panicking.rs:142
    //    2: core::panicking::panic_bounds_check
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library\core\src\panicking.rs:84
    //    3: core::slice::index::impl$2::index<i32>
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc\library\core\src\slice\index.rs:242
    //    4: core::slice::index::impl$0::index<i32,usize>
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc\library\core\src\slice\index.rs:18
    //    5: alloc::vec::impl$16::index<i32,usize,alloc::alloc::Global>
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc\library\alloc\src\vec\mod.rs:2591
    //    6: learn_rust_book::chapter_009::chapter_009_1
    //              at .\src\chapter_009.rs:61
    //    7: learn_rust_book::chapter_009::run
    //              at .\src\chapter_009.rs:3
    //    8: learn_rust_book::main
    //              at .\src\main.rs:21
    //    9: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc\library\core\src\ops\function.rs:248
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    // error: process didn't exit successfully: `target\debug\learn-rust-book.exe` (exit code: 101)

    // 5
    // When running with...
    // RUST_BACKTRACE=1 cargo run --release
    //
    // let v = vec![1, 2, 3];
    // let _v_100 = v[100];
    //
    // ... produces the following output:
    //
    // Chapter 9: Error Handling
    // Chapter 9.1: Unrecoverable errors with panic
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 100', src\chapter_009.rs:61:18
    // stack backtrace:
    //    0: std::panicking::begin_panic_handler
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library\std\src\panicking.rs:584
    //    1: core::panicking::panic_fmt
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library\core\src\panicking.rs:142
    //    2: core::panicking::panic_bounds_check
    //              at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library\core\src\panicking.rs:84
    //    3: learn_rust_book::chapter_009::run
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    // error: process didn't exit successfully: `target\release\learn-rust-book.exe` (exit code: 101)
}
