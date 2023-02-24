use std::{thread, time::Duration, sync::{mpsc, Mutex, Arc}};

pub fn run() {
    println!("16. Fearless Concurrency");
    _chapter_016_1();
    _chapter_016_2();
    _chapter_016_3();
    _chapter_016_4();
}

fn _chapter_016_1() {
    println!("16.1. Using Threads to Run Code Simultaneously");
    
    // Creating thread using thread::spawn
    {
        std::thread::spawn(|| {
            for i in 0..20 {
                println!("spawned thread, number {}...", i);
                thread::sleep(Duration::from_millis(10));
            }
        });
        
        for i in 0..5 {
            println!("main thread, number {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    
        println!("main thread finished");
    }
    
    // Waiting for All Threads to Finish Using join Handles
    {
        let hangle = std::thread::spawn(|| {
            for i in 0..20 {
                println!("spawned thread, number {}...", i);
                thread::sleep(Duration::from_millis(5));
            }
        });
        
        for i in 0..5 {
            println!("main thread, number {}", i);
            thread::sleep(Duration::from_millis(5));
        }

        println!("main thread finished");
        
        hangle.join().unwrap();
    }

    // Using move Closures with Threads
    {
        // let v = vec![1, 2, 3];
        // // Error:
        // // Closures body only borrows v, and may outlive parent thread
        // let handle = thread::spawn(|| {
        //     println!("Here's a vector: {:?}", v);
        // });
        // handle.join().unwrap();

        let v = vec![1, 2, 3];
        // Closure body moves caputured value, parent thread is not an owner anymore.
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();
    }

}

fn _chapter_016_2() {
    println!("16.2. Using Message Passing to Transfer Data Between Threads");

    // Using Message Passing to Transfer Data Between Threads
    fn mpsc_with_simple_msg_passing()
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let value = String::from("Hello World from thread");
            tx.send(value).unwrap();
        });

        let received_value = rx.recv().unwrap();
        println!("received value: {}", received_value);
    }

    // Sending Multiple Values and Seeing the Receiver Waiting
    fn mpsc_with_simple_mult_msg_passing()
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let values = vec![
                String::from("Hello"),
                String::from("World"),
                String::from("!"),
                String::from("ABC"),
            ];

            for value in values {
                println!("spawned thread: about to send: {}...", value);
                tx.send(value).unwrap();
                thread::sleep(Duration::from_millis(20));
            }
        });

        for received_value in rx {
            println!("main thread: received value: {}", received_value);
        }
    }

    // Creating Multiple Producers by Cloning the Transmitter
    fn mpsc_with_cloning_tx() 
    {
        let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let values = vec![
                String::from("Hello"),
                String::from("World"),
                String::from("!"),
                String::from("ABC"),
            ];

            for value in values {
                println!("\tspawned thread: tx about to send: {}...", value);
                tx.send(value).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });

        thread::spawn(move || {
            let values = vec![
                String::from("Hello"),
                String::from("World"),
                String::from("!"),
                String::from("ABC"),
            ];

            for value in values {
                println!("\tspawned thread: tx_clone about to send: {}...", value);
                tx_clone.send(value).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });

        for received_value in rx {
            println!("main thread: received value: {}", received_value);
        }        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let values = vec![
                String::from("Hello"),
                String::from("World"),
                String::from("!"),
                String::from("ABC"),
            ];

            for value in values {
                println!("spawned thread: about to send: {}...", value);
                tx.send(value).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });

        for received_value in rx {
            println!("main thread: received value: {}", received_value);
        }
    }

    // Sending Multiple Values and Seeing the Receiver handles errors
    fn mpsc_with_errors() 
    {
        let (tx, rx) = mpsc::channel();

        let _handle = thread::spawn(move || {
            let values = vec![
                String::from("Message 1"),
                String::from("Message 2"),
                String::from("Message 3"),
                String::from("Message 4"),
                String::from("Message 5"),
            ];

            for value in values {
                println!("spawned thread: about to send: {}...", value);
                tx.send(value).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
            
            thread::sleep(Duration::from_millis(1000));

            tx.send("Message 6".to_string()).unwrap();
        });

        let result = loop {

            let received_value = rx.try_recv();

            match received_value {
                Ok(value) => {
                    println!("received value: {}", value);
                },
                Err(e) => {
                    match e {
                        mpsc::TryRecvError::Empty => {
                            println!("received error: {:?}, about to sleep for 100 ms...", e);
                        },
                        mpsc::TryRecvError::Disconnected => {
                            println!("received error: {:?}", e);
                            break 100;
                        },
                    }
                    thread::sleep(Duration::from_millis(200));
                }
            }
        };

        println!("result: {:?}", result);
    }

    // Sending Multiple Values and Seeing the Receiver handles errors using iterators
    fn mpsc_with_errors_and_itors() 
    {
        let (tx, rx) = mpsc::channel();

        let _handle = thread::spawn(move || {
            let values = vec![
                String::from("Message 1"),
                String::from("Message 2"),
                String::from("Message 3"),
                String::from("Message 4"),
                String::from("Message 5"),
            ];

            for value in values {
                println!("spawned thread: about to send: {}...", value);
                tx.send(value).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
            
            thread::sleep(Duration::from_millis(1000));

            tx.send("Message 6".to_string()).unwrap();
        });


        let mut iter = rx.iter();

        // // Compact version
        // while let Some(value) = iter.next() {
        //     println!("received value from iterator: {}", value);
        // }
        
        // Expanded versino
        loop {
            match iter.next() {
                Some(value) => {
                    println!("received value from iterator: {}", value);
                }
                None => {
                    println!("received None from iterartor");
                    break;
                },
            }
        }
    }

    mpsc_with_simple_msg_passing();
    mpsc_with_simple_mult_msg_passing();
    mpsc_with_cloning_tx();
    mpsc_with_errors();
    mpsc_with_errors_and_itors();
}

fn _chapter_016_3() {
    println!("16.3. Shared-State Concurrency");

    // The API of Mutex<T>
    {
        fn print_mutex_guard_value(m_value: &i32) {
            println!("print_mutex_guard_value: m_value: {:?}", m_value)
        }

        let m = Mutex::new(100);

        // Expanded
        {
            let m_l = m.lock();

            match m_l {
                Ok(m_g) => {
                    println!("m_g: {:?}", m_g);

                    // Access value directly
                    let m_value = *m_g;
                    println!("m_value: {:?}", m_value);

                    // Access value using deref coercion
                    print_mutex_guard_value(&m_g);
                },
                Err(e) => {
                    println!("e: {:?}", e);
                },
            }

        }
        
        // Compact
        {
            // Access value via .unwrap
            let mut m_value = m.lock().unwrap();
            *m_value = 500;
        }

        println!("m: {:?}", m);
    }

    // Sharing a Mutex<T> Between Multiple Threads
    {
        let counter = Arc::new(Mutex::new(0));

        let mut handles = vec![];

        for _ in 0..20 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut m = counter.lock().unwrap();
                *m += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("counter: {:?}", counter);
        let counter_value = *(counter.lock().unwrap());
        println!("counter_value: {:?}", counter_value);
    }

    // Emulate deadlocks
    {
        let counter = Arc::new(Mutex::new(0));
        let counter_for_t2 = Arc::clone(&counter);

        let t1 = thread::spawn(move || {
            println!("t1: {:?}", counter.lock().unwrap());
        });

        let t2 = thread::spawn(move || {
            println!("t2: {:?}", counter_for_t2.lock().unwrap());
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
}

fn _chapter_016_4() {
    println!("16.4. todo...");
}
