pub fn run () {
    println!("4. Understanding Ownership");
    chapter_4_1();
    chapter_4_2();
    chapter_4_3();
}

fn chapter_4_1 () {
    println!("4.1. What is Ownership");

    // Mutate string
    let mut s = String::from("String");
    s.push_str(" + string");
    println!("s: {}", s);

    // Simple String move
    let s1 = String::from("abc");
    let s2 = s1;
    println!("s2: {}", s2);
    
    // Simple String copy
    let s1 = String::from("abc");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    // Stack-only data - copy
    let i1 = 100;
    let i2 = i1;
    println!("i1: {}, i2: {}: ", i1, i2);

    // Ownership and functions
    let s1 = String::from("another_abc");
    takes_ownership(s1);
    makes_copy(i2);

    // Return Values and Scope
    let s1 = gives_ownership();
    let s2 = String::from("let s2");
    let s3 = takes_and_gives_back(s2);
    println!("s1: {}, s2: invalidated, s3: {}: ", s1, s3);
}

fn takes_ownership(s: String) {
    println!("s: {}", s);
}

fn makes_copy(x: i32) {
    println!("x: {}", x);
}

fn gives_ownership() -> String {
    let s = String::from("From gives_ownership");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

// ----------------------------------------------------------------

fn chapter_4_2 () {
    println!("4.2. References and Borrowing");

    let s = String::from("abc");
    let l = calculate_length(&s);
    println!("s: {}, l: {}", s, l);

    // Mutable Reference
    let mut s = String::from("abc");
    println!("Before change: {}", s);
    change(&mut s);
    println!("After change: {}", s);

    // Borrowing few mutable references
    let r1 = &mut s;
    println!("r1: {}", r1);
        // let r2 = &mut s; // Error: cannot borrow s as mutable more than once at the time;
        //println!("r1: {}, r2: {}", r1, r2);
    {
        let r2 = &mut s; // is fine because r2 is scoped.
        println!("r2: {}", r2);
    }

    let r3 = &s;
    let r4 = &s;
    // let r5 = &mut s; // Error, someone can modify s using r5, so, as r3 and r4 are immutable, it's unexpected modification for them.
    println!("r3: {}, r4: {}", r3, r4);
    
    // Reference Scope
    let r3 = &s;
    let r4 = &s;
    println!("r3: {}, r4: {}", r3, r4); // The scope of r3 and r4 ends here.
    let r5 = &mut s; // OK, there is no one else refers to s. The scope of r3 and r4 ends after println.
    println!("r5: {}", r5);

    // Test dangling referrence
    // test_dangle();
}

fn calculate_length(s: &String) -> usize {
    return s.len()
}

fn change(s: &mut String) {
    s.push_str("`_push_str`-ed");
}

// fn test_dangle() -> &String { // Error: expected named lifetime parameter
//     let s = String::from("abc");
//     &s
// }

use std::str::FromStr;

fn chapter_4_3 () {
    println!("4.3. The Slice Type");
    let s = String::from("abc def ghj");
    let first = first_word(&s);
    // s.clear(); // Error: cannot borrow s as mutable more than once...
    println!("first: {}", first);

    let strings = ["", 
                            "one_word", 
                            "two  words", 
                            "three words here", 
                            "   ", 
                            " a "];
    let nth = 0;
    for s in strings {
        let test_s = String::from_str(&s).unwrap();
        let w = nth_word(&test_s, nth);
        println!("s: {}, nth: {}, w: {}", s, nth, w);
    }

        // Other Slices
        let a = [0, 1, 2, 3, 4, 5];
        let a_sl = &a[1..4];
        // let a_sl = &[1..4]; // TODO: Ask about this construction
        println!("a: {:?}, a_sl: {:?}", a, a_sl);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..]
}

/// nth_word - TODO: Return the nth word in a string
fn nth_word(s: &String, n: i32) -> &str {
    let bytes = s.as_bytes();
    let mut space_count = 0;
    let mut from = 0;
    let to = bytes.len();

    for (i, &item) in bytes.iter().enumerate() {
        
        if item == b' ' {
            println!("Space at: {}, nth: {}", i, space_count);

            if from == 0 {
            }

            space_count += 1;
        }

        if space_count == n {
            from = i;
        }
    }
    
    return &s[from..to]
}
