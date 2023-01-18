pub fn run() {
    chapter_013_1();
    chapter_013_2();
    chapter_013_3();
    chapter_013_4();
}

#[allow(dead_code)]
fn chapter_013_1() {
    println!("Chapter 13.1 - Closures: Anonymous Functions that Capture Their Environment");

    // Capturing the Environment with Closures
    {
        #[derive(Debug, PartialEq, Clone, Copy)]
        enum ShirtColor {
            Red,
            Blue,
        }

        struct Inventory {
            shirts: Vec<ShirtColor>,
        }

        impl Inventory {
            fn giveaway(&self, user_preferences: Option<ShirtColor>) -> ShirtColor {
                user_preferences.unwrap_or_else(|| self.most_stocked())
            }

            fn most_stocked(&self) -> ShirtColor {
                let mut num_red = 0;
                let mut num_blue = 0;

                for color in &self.shirts {
                    match color {
                        ShirtColor::Red => num_red += 1,
                        ShirtColor::Blue => num_blue += 1,
                    }
                }

                if num_red > num_blue {
                    ShirtColor::Red
                } else {
                    ShirtColor::Blue
                }
            }
        }

        fn test_inventory() {
            let store = Inventory {
                shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
            };
        
            let user_pref1 = Some(ShirtColor::Red);
            let giveaway1 = store.giveaway(user_pref1);
            println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);
        
            let user_pref2 = None;
            let giveaway2 = store.giveaway(user_pref2);
            println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

            let user_pref3 = Some(ShirtColor::Blue);
            let giveaway3 = store.giveaway(user_pref3);
            println!("The user with preference {:?} gets {:?}", user_pref3, giveaway3);
        }

        test_inventory()
    }

    // Closure Type Inference and Annotation
    {
        let expensive_closure_1 = |num: u64| -> u64 {
            println!("payload: {:?}", num);
            std::thread::sleep(std::time::Duration::from_millis(10));
            num
        };
        expensive_closure_1(10);

        fn add_one_v1(i: u64) -> u64 { i + 1 }
        println!("add_one_v1: {:?}", add_one_v1(10));
        
        let add_one_v2 = |i: u64| -> u64 { i + 1 };
        println!("add_one_v2: {:?}", add_one_v2(10));

        // let add_one_v3 = |i| -> { i + 1 }; // Error: Expected type
        // println!("add_one_v3: {:?}", add_one_v3(10));
        
        // Ok or Error case:
        // let add_one_v4 = |i| i + 1; // Error: Expected type. And no error if add_one_v4 will be called below
        // println!("add_one_v4: {:?}", add_one_v4(10));

        let example_closure = |x| x;
        let _r1 = example_closure(1);
        let _r2 = example_closure(2);
        
        // The code below
        // 
        // let r3 = example_closure("3");
        //
        // Generates the following output:
        // mismatched types expected integer, found `&str`
    }

    // Capturing References or Moving Ownership
    {
        // Borrow immutable
        let v = vec![1, 2, 3];
        println!("[borrow immutable] before defining closure: {:?}", v);
        let v_print = || println!("From v_print: {:?}", v);
        println!("[borrow immutable] before calling closure: {:?}", v);
        v_print();
        println!("[borrow immutable] after calling closure: {:?}", v);
        
        // Borrow mutable
        let mut v = vec![1, 2, 3];
        println!("[borrow mutable] before defining closure: {:?}", v);
        let mut v_print = || v.push(100);
        
        // Error: cannot borrow immutable because it is already borrowed as mutable (in the body of closure)
        // println!("[borrow mutable] before calling closure: {:?}", v);
        v_print(); // Immutable borrow ends here.
        println!("[borrow mutable] after calling closure: {:?}", v);

        // Move Ownership (1)
        let v = vec![1, 2, 3];
        println!("[move ownership] before defining closure: {:?}", v);
        let v_print = move || println!("From v_print: {:?}", v);

        // The code below:
        // println!("[move ownership] before calling closure: {:?}", v);
        // Generate the following output:
            // error[E0382]: borrow of moved value: `v`
            // --> src/chapter_013.rs:128:67
            //     |
            // 123 |         let v = vec![1, 2, 3];
            //     |             - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
            // 124 |         println!("[move ownership] before defining closure: {:?}", v);
            // 125 |         let v_print = move || println!("From v_print: {:?}", v);
            //     |                       ------- value moved into closure here  - variable moved due to use in closure
            // ...
            // 128 |         println!("[move ownership] before calling closure: {:?}", v);
            //     |                                                                   ^ value borrowed here after move
        v_print();

        // Move Ownership (2)
        {   
            let v = vec![1, 2, 3];
            println!("[move ownership] before defining closure: {:?}", v);

            let thread_handle = std::thread::spawn(move || {
                println!("From thread: {:?}", v);
                "thread finished".to_owned()
            });
            println!("[move ownership(2)] thread handle: {:?}", thread_handle);

            let thread_result = thread_handle.join();
            println!("[move ownership(2)] thread result: {:?}", thread_result);

            let thread_result_value = thread_result.unwrap_or("This will be shown if thread ends with an error".to_owned());
            println!("[move ownership(2)] thread result value: {:?}", thread_result_value);

            std::thread::spawn(move || println!("From second thread..."))
                .join()
                .unwrap();
        }

        // Moving Captured Values Out of Closures and the Fn Traits
        {
            //
            // 1
            //
            {
                #[derive(Debug)]
                enum MyOption<T> {
                    Some(T),
                    None,
                }
    
                impl <T> MyOption<T> {
                    
                    pub fn unwrap_or_else<F>(self, f: F) -> T
                        where
                            F: FnOnce() -> T
                    {
                        match self {
                            MyOption::Some(value) => value,
                            MyOption::None => f(),
                        }
                    }
                }
    
                let mo1 = MyOption::Some(100);
                let mo2: MyOption<f64> = MyOption::None;
                println!("mo1: {:?}", mo1);
                println!("mo2: {:?}", mo2);
    
                let mo1_value = mo1.unwrap_or_else(|| {
                    println!("Error: mo1 is None");
                    -1
                });
                let mo2_value = mo2.unwrap_or_else(|| 12.34);
    
                println!("mo1_value: {:?}", mo1_value);
                println!("mo2_value: {:?}", mo2_value);
            }

            //
            // 2
            //
            {
                use learn_rust_book::rust_book::chapter_005::Rectangle;

                let mut v = vec![
                    Rectangle::new(10, 20), 
                    Rectangle::new(3, 4),
                    Rectangle::new(5, 1),
                ];
                println!("[1] v: {:?}", v);
                
                v.sort_by_key(|r| r.width);
                
                println!("[2] v: {:?}", v);
                
                let mut num_sorts = 0;
                v.sort_by_key(|r| {
                    num_sorts += 1;
                    r.height
                });
                
                println!("[3] v: {:?}, num_sorts: {num_sorts}", v);

                let mut v = vec![10, 20, 10, 5, 4];
                println!("[4] v: {:?}", v);
                v.sort_by(|a, b|
                    a.cmp(b)
                );
                println!("[5] v: {:?}", v);
            }
        }
    }
}

#[allow(dead_code)]
fn chapter_013_2() {
    println!("Chapter 13.2 - Processing a Series of Items with Iterators");

    {
        let v_i = vec![5, 10, 15];
        for val in v_i.iter() {
            println!("value: {:?}", val);
        }
    
        let mut v_i_iter = v_i.iter();
    
        assert_eq!(v_i_iter.next(), Some(&5));
        assert_eq!(v_i_iter.next(), Some(&10));
        assert_eq!(v_i_iter.next(), Some(&15));
        assert_eq!(v_i_iter.next(), None);

    }

    {
        // Using iter
        let v_i = vec![1, 2, 3, 4, 5];
        println!("[1] v_i: {:?}", v_i);
        
        let mut v_i_iter = v_i.iter();
        let value = v_i_iter.next();
        println!("[1] value: {:?}", value);
        
        println!("[1] v_i: {:?}", v_i);

        // Using iter_mut
        let mut v_i = vec![1, 2, 3, 4, 5];
        println!("[2] v_i: {:?}", v_i);

        {
            let mut v_i_iter = v_i.iter_mut();
            let value = v_i_iter.next().unwrap();
            *value = 54321;
            println!("[2] value: {:?}", value);
        }

        println!("[2] v_i: {:?}", v_i);    
    }

    // Using into_iter (1)
    {
        let v_i = vec![1, 2, 3, 4, 5];
        println!("[3] v_i: {:?}", v_i);

        let mut v_i_iter = v_i.into_iter();
        let value = v_i_iter.next();
        println!("[3] value: {:?}", value);

        // Error: borrow of moved value: `v_i`
        // println!("[3] v_i: {:?}", v_i);
    }

    // Using into_iter (2)
    {
        let v_i = vec![1, 2, 3, 4, 5];
        println!("[4] v_i: {:?}", v_i);

        for value in v_i.into_iter() {
            if value == 3 {
                break;
            }
        }

        // Error: borrow of moved value: `v_i`
        // println!("[4] v_i: {:?}", v_i);
    }

    // Methods that Consume the Iterator
    {
        let v_i = vec![1, 2, 3, 4, 5];

        let v_i_iter = v_i.iter();

        let v_i_sum: i32 = v_i_iter.sum();
        println!("v_i: {:?}", v_i);
        println!("v_i_sum: {}", v_i_sum)
    }

    // Methods that Produce Other Iterators
    {
        let v_i = vec![1, 2, 3, 4, 5];

        let v_i_iter = v_i.iter();
        
        let v_i_adaptor = v_i_iter.map(|i| i * 10);

        let v_i_mapped: Vec<i32> = v_i_adaptor.collect();

        println!("v_i: {:?}", v_i);
        println!("v_i_mapped: {:?}", v_i_mapped);
        println!("v_i_mapped (i * 100): {:?}", v_i.iter().map(|i| i * 100).collect::<Vec<i32>>());
    }

    // Using Closures that Capture Their Environment
    {
        use learn_rust_book::rust_book::chapter_013::Shoe;

        let v_in = vec![
            Shoe::new(42, "style_001".to_owned()),
            Shoe::new(43, "style_001".to_owned()),
            Shoe::new(44, "style_001".to_owned()),
        ];
        println!("v_in: {:?}", v_in);
        
        let v_out: Vec<Shoe> = Shoe::shoes_by_size(v_in, 43);
        println!("v_out: {:?}", v_out);
    }

    // Playing with filter
    {
        let v_in = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        println!("v_in: {v_in:?}");

        let v_out: Vec<&i32> = v_in.iter().filter(|&&i| {
            (i % 2) == 0
        }).collect();
        println!("v_out: {v_out:?}");
        
        let v_out: Vec<i32> = v_out.iter().map(|&&i| {
            ((i + i) as f64 * 1.5) as i32
        }).collect();
        println!("v_out: {v_out:?}");

        let mut v_in_mut = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        println!("v_in_mut: {v_in_mut:?}");
        
        let v_out = v_in_mut.iter_mut().for_each(|i| {
            *i += 1000;
        });
        
        println!("v_in_mut: {v_in_mut:?}");
        println!("v_out: {v_out:?}");

        let v_in = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let v_out: Vec<&i32> = v_in.iter().filter_map(|i| {
            if i % 3 == 0 {
                Some(i)
            } else {
                None
            }
        }).collect();
        println!("v_in: {v_in:?}");
        println!("v_out: {v_out:?}");
    }
}

fn chapter_013_3() {
    println!("Chapter 13.3 - Improving Our I/O Project");
    {
        use std::env;
        use std::process;
        
        use learn_rust_book::chapter_013_lib::Config;
        
        fn run_main() {
            let config = Config::build(env::args()).unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: {err}");
                process::exit(1);
            });

            println!("Searching for {}", config.query);
            println!("In file {}", config.file_path);

            if let Err(e) = learn_rust_book::chapter_013_lib::run(&config) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }

        run_main();
    }
}

fn chapter_013_4() {
    println!("Chapter 13.4 - Comparing Performance: Loops vs. Iterators");
    
    // Trying different constructions
    {
        let v_i = vec![10, 20, 30, 40, 50, 60,];
        let mut _buffer = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        for i in 12.._buffer.len() {
            println!("i: {:?}", i);
            let i_range = i - 12..i;
            println!("i - 12..i: {:?}", i_range);
            println!("\n")
        }

        // Playing with zip
        let v_i_mapped: Vec<i32> = v_i
            .iter()
            .map(|&i| {
                i + 1000
            })
            .collect();

        let v_out_zipped_mapped_iter_sep: Vec<(&i32, &i32)> = _buffer
            .iter()
            .zip(v_i_mapped.iter())
            .collect();
        println!("v_out_zipped_mapped_iter_sep: {v_out_zipped_mapped_iter_sep:?}");

        let v_out_zipped_mapped_iter_inline = _buffer
            .iter()
            .zip(v_i_mapped.iter())
            .collect::<Vec<(&i32, &i32)>>();
        println!("v_out_zipped_mapped_iter_inline: {v_out_zipped_mapped_iter_inline:?}");
    }
}

#[cfg(test)]
mod tests_ch13_2 {
    #[test]
    fn filter_by_size() {
        use learn_rust_book::rust_book::chapter_013::Shoe;

        let v_in = vec![
            Shoe::new(42, "style_001".to_owned()),
            Shoe::new(43, "style_001".to_owned()),
            Shoe::new(44, "style_001".to_owned()),
        ];
        
        let v_out: Vec<Shoe> = Shoe::shoes_by_size(v_in, 43);

        assert_eq!(vec![Shoe::new(43, "style_001".to_owned())],v_out)
    }
}
