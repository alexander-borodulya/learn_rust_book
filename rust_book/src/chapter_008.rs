pub fn run() {
    println!("8. Common Collections");
    chapter_008_1();
    chapter_008_2();
    chapter_008_3();
}

fn chapter_008_1() {
    println!("8.1. Storing Lists of Values with Vectors");

    // Creating a New Vector
    {
        let _v_of_i32: Vec<i32> = Vec::new();
        let _or_vec_of_f64: Vec<f64> = Vec::new();
        println!("_v_of_i32: {:?}", _v_of_i32);
        println!("_or_vec_of_f64: {:?}", _or_vec_of_f64);
        
        // Using vec! macro...
        let _m_v = vec![1, 2, 3, 4, 5];
        let _m_usize = vec![1_000_000_000usize, 2, 3, 4, 5];
        
        println!("_m_v: {:?}", _m_v);
        println!("_m_usize: {:?}", _m_usize);
    }
    
    // Updating a Vector
    {
        let mut v = Vec::new();
        println!("v (1): {:?}", v);
        v.push(1);
        v.push(12);
        println!("v (2): {:?}", v);
    }
    
    // Reading Elements of Vectors
    #[allow(unused_assignments)]
    {
        println!();

        // Using indices

        let v = vec![1, 2, 3, 4, 5, 6];
        println!("v (1): {:?}", v);

        let v_i_3 = v[3];
        println!("v_i_3: {:?}", v_i_3);

        let v_i_3_ref = &v[3];
        println!("v_i_3_ref: {:?}", v_i_3_ref);

        // Using get
        
        // 3
        let mut v_index = 3;
        {
            let v_i_3_get = v.get(v_index);
            println!("v_i_3_get: {:?}", v_i_3_get);
    
            match v_i_3_get {
                Some(value) => {
                    println!("v_i_3_get: {:?}", value);
                },
                None => {
                    println!("v_i_3_get: None: Bad index: {v_index}");
                },
            }
        }

        // 1003
        v_index = 1003;
        {
            let v_val = v.get(v_index);
            println!("v_val[{v_index}] = {:?}", v_val);
    
            match v_val {
                Some(value) => {
                    println!("v_val: {:?}", value);
                },
                None => {
                    println!("v_val[{v_index}]: None: Bad index: {v_index}");
                },
            }
        }

        // With mutable reference
        {
            println!();

            let mut v = vec![100, 200, 300, 400, 500];
            println!("mut v: {:?}", v);
            
            let v_i_1 = &v[1];
            println!("v_i_1 (1): {:?}", v_i_1);
            
            // Calling this...
            //
            // v.push(1000); // <- Modification happened here
            // println!("v_i_1 (2): {:?}", v_i_1); // <-- Reads immutable reference to the value after the modification
            //
            // Generates the following output:
            //
            // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
            //   --> src\chapter_008.rs:95:13
            //    |
            // 92 |             let v_i_1 = &v[1];
            //    |                          - immutable borrow occurs here
            // ...
            // 95 |             v.push(1000);
            //    |             ^^^^^^^^^^^^ mutable borrow occurs here
            // 96 |             println!("mut v: {:?}", v);
            // 97 |             println!("v_i_1 (2): {:?}", v_i_1);
            //    |                                         ----- immutable borrow later used here
            v.push(10_000);
        }

        // Iterating over the Values in a Vector
        {
            println!();

            let v = vec![10, 20, 30, 40, 50];
            // for i in v { <-- Move of v
            for i in &v { // Borrow of v
                println!("i: {:?}", i);
            }
            println!("v, print 1: {:?}", v); // Safe to use v, because it was borrowed...
            println!("v, print 2: {:?}", v); // Safe to use v, because it was borrowed...

            // Mutate...
            let mut v_mut = vec![10, 20, 30, 40, 50];
            println!("v_mut (1): {:?}", v_mut);
            
            for i in &mut v_mut {
                *i = *i **i **i;
            }

            println!("v_mut (2): {:?}", v_mut);
        }

        // Using an Enum to Store Multiple Types
        {
            println!();

            #[derive(Debug)]
            enum BasicTypes {
                I32(i32),
                F32(f32),
                Text(String),
            }

            let v = vec![
                BasicTypes::I32(100), 
                BasicTypes::F32(200.200),
                BasicTypes::Text("String.to_owned".to_owned())];

            println!("v: {:?}", &v);
            // let v_0 = v[0]; // Can't move: Doesn't implement Copy trait...
            println!("v: {:?}", v);

            for i in &v {
                println!("iterate: i: {:?}", i);
            }

            //
            // 
            //

            #[derive(Debug)]
            enum BasicTypeStringSlice <'a> {
                TextSlice(&'a str),
            }

            let v1 = vec![BasicTypeStringSlice::TextSlice("String")];
            println!("v1: {:?}", v1);

            let str_slice = "12345";
            let v2 = vec![BasicTypeStringSlice::TextSlice(str_slice)];
            println!("v2: {:?}", v2);

            //
            //
            //

            #[derive(Debug)]
            enum BasicTypesCombined<'a> {
                USize(usize),
                StringSlice(&'a str),
                Any(())
            }
            let v3 = vec![BasicTypesCombined::USize(321), BasicTypesCombined::StringSlice("s_l_i_c_e")];
            println!("v3: {:?}", v3);

            let mut btc: BasicTypesCombined = BasicTypesCombined::Any(());

            match btc {
                BasicTypesCombined::USize(us) => println!("BasicTypesCombined::USize: {:?}", us),
                BasicTypesCombined::StringSlice(s) => println!("BasicTypesCombined::StringSlice: {:?}", s),
                BasicTypesCombined::Any(a) => println!("BasicTypesCombined::Any: {:?}", a),
            }

            btc = BasicTypesCombined::StringSlice("( . )( . )");

            match btc {
                BasicTypesCombined::USize(us) => println!("BasicTypesCombined::USize: {:?}", us),
                BasicTypesCombined::StringSlice(s) => println!("BasicTypesCombined::StringSlice: {:?}", s),
                BasicTypesCombined::Any(a) => println!("BasicTypesCombined::Any: {:?}", a),
            }
        }
    }
}

fn chapter_008_2() {
    println!("8.2.");

    // Creating a new string
    {
        let data = "12345";
        let s = data.to_string();
        let s2 = "54321".to_string();
        let s3 = String::from("From String");
        println!("data: {:?}", data);
        println!("s: {:?}", s);
        println!("s2: {:?}", s2);
        println!("s3: {:?}", s3);
    }

    // Updating a String
    {
        let mut s = String::from("From String");
        s.push_str(" + string slice");
        println!("s: {:?}", s);

        let mut s1 = String::from("From String");
        let str_slice = "12345";
        s1.push_str(str_slice);
        println!("s1, str_slice: {:?}, {:?}", s1, str_slice);
        
        s1.push('_');
        s1.push('A');
        s1.push('_');
        s1.push('B');

        println!("s1, str_slice: {:?}, {:?}", s1, str_slice);
    }

    // Concatenation with the + Operator or the format! Macro
    {
        let s1 = String::from("From String");
        let s2 = String::from(" Another String");
        let s3 = s1 + &s2; // s1 value borrowed here after move
        println!("s2: {:?}, s3: {:?}", s2, s3);

        {
            #[derive(Debug)]
            struct Object {
                s: String,
            }

            impl Object {
                pub fn new(s: String) -> Self { 
                    Object { s }
                }

                pub fn add(self, s_in: String) -> Self {
                    Self { s: self.s + &s_in }
                }
            }

            let o1 = Object::new("object 1".to_owned());
            let o2 = Object::new("object 2".to_owned());
            println!("o1: {:?}, o2: {:?}", o1, o2);

            let o3 = o1.add(" 12345".to_owned());
            println!("02: {:?}", o2);
            println!("o3: {:?}", o3);
        }

        let s1 = "Redeclared s1";
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s: {:?}", s);
    }

    // Indexing into Strings
    {
        let _s1 = String::from("Something");
        //
        // Calling this...
        //
        //let h = _s1[0];
        //
        // Generates the following output:
        //
        // error[E0277]: the type `String` cannot be indexed by `{integer}`
        //         --> src\chapter_008.rs:285:17
        //          |
        //      285 |         let h = s1[0];
        //          |                 ^^^^^ `String` cannot be indexed by `{integer}`
        //          |
        //          = help: the trait `Index<{integer}>` is not implemented for `String`
        //          = help: the following other types implement trait `Index<Idx>`:
        //                    <String as Index<RangeFrom<usize>>>
        //                    <String as Index<RangeFull>>
        //                    <String as Index<RangeInclusive<usize>>>
        //                    <String as Index<RangeTo<usize>>>
        //                    <String as Index<RangeToInclusive<usize>>>
        //                    <String as Index<std::ops::Range<usize>>>
        //                    <str as Index<I>>
            
        //      For more information about this error, try `rustc --explain E0277`.
        //      error: could not compile `learn-rust-book` due to previous error
    }

    // Internal Representation
    {
        let s = String::from("Something");
        let h = &s[0..1];
        println!("s: {}", s);
        println!("h: {}", h);

        //
        // Calling this...
        //
        // let s = String::from("Фигня");
        // let h = &s[0..1];
        // println!("s: {}", s);
        // println!("h: {}", h);
        //
        // Generates the following output:
        //
        // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'Ф' (bytes 0..2) of `Фигня`', library\core\src\str\mod.rs:127:5
        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        // error: process didn't exit successfully: `target\debug\learn-rust-book.exe` (exit code: 101)
    }

    // Slicing Strings
    {
        let s = String::from("Уже что-то лучше");
        let h = &s[0..4];
        println!("s: {}", s);
        println!("h: {}", h);
    }

    // Methods for Iterating Over Strings
    {
        let s1 = "Something";
        let s2 = "Что-то новенькое";

        for c in s1.chars() {
            println!("chars: s1: {}", c);
        }

        for c in s2.chars() {
            println!("chars: s2: {}", c);
        }

        for c in s1.bytes() {
            println!("bytes: s1: {}", c);
        }

        for c in s2.bytes() {
            println!("bytes: s2: {}", c);
        }

        println!("s1.len: {}", s1.len());
        println!("s2.len: {}", s2.len());
        // GetSize(s1);
        // println!("s1.size: {}", s1.size());
        // println!("s2.size: {}", s2.size());
    }
}

fn chapter_008_3() {
    println!("8.3. Storing Keys with Associated Values in Hash Maps");

    // Creating a new HashMap
    {
        
        // Simple HashMap
        {
            use std::collections::HashMap;

            let mut hm = HashMap::new();
            hm.insert("A", 1);
            hm.insert("B", 2);
            hm.insert("CDE", 345);
            println!("hm: {:?}", hm);
        }
        
        // Expanded version
        {
            use std::collections::HashMap;
            
            let teams = vec!["Team1", "Team2", "Team3"];
            let scores = vec![100, 200, 300];
            
            let teams_iter = teams.into_iter();
            let scores_iter = scores.into_iter();
            
            let zipped = teams_iter.zip(scores_iter);
            let teams_map: HashMap<_, _> = zipped.collect();
            
            println!("teams: {:?}", teams_map);
        }
        
        // Short for Teams
        {
            use std::collections::HashMap;

            let teams = vec!["Team1", "Team2", "Team3"];
            let scores = vec![100, 200, 300];

            let teams_map: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();
            println!("teams: {:?}", teams_map);
        }

        // Short for Scores
        {
            use std::collections::HashMap;
            
            let teams = vec!["Team1", "Team2", "Team3"];
            let scores = vec![100, 200, 300];

            let scores_map: HashMap<_, _> = scores.into_iter().zip(teams.into_iter()).collect();
            println!("scores: {:?}", scores_map);
        }

        // Hash Maps and Ownership
        {
            use std::collections::HashMap;
    
            // Owned types
            {
                let k = String::from("Filename");
                let v = String::from("FieldValue");
        
                let mut m = HashMap::new();
                m.insert(k, v); // k, v - value moved here
        
                println!("m: {:?}", m);
                // println!("k, v: {:?}, {:?}", k, v); // k, v, ^ value borrowed here after move
            }
    
            // Types that implements Copy trait
            {
                let k = 100;
                let v = "str slice";
                let mut m = HashMap::new();
                m.insert(k, v);
                println!("m: {:?}", m);
                println!("k, v: {:?}, {:?}", k, v); // Ok
            }
        }
    
        // Accessing Values in Hash Map
        {
            use std::collections::HashMap;
    
            let mut scores = HashMap::new();
            scores.insert(String::from("key_a"), 100);
            scores.insert(String::from("key_b"), 200);
    
            let team_name = "key_a".to_owned();
            let key_a_value = scores.get(&team_name);
            println!("key_a_value: {:?}", key_a_value);
    
            let key_c_value = "key_c".to_owned();
            let key_c_value = scores.get(&key_c_value);
            println!("key_c_value: {:?}", key_c_value);
    
            // Handle some
            if let Some(v) = key_a_value {
                println!("Actual value: {:?}", v);
            }
    
            // Iterating in for loop
            for (k, v) in &scores {
                println!("{:?}: {:?}", k, v);
            }
        }
    }

    // Updating a HashMap
    {
        use std::collections::HashMap;

        // Overwriting a Value
        {
            let mut scores = HashMap::new();
            scores.insert(String::from("key_a"), 100);
            scores.insert(String::from("key_a"), 200);
            println!("scores: {:?}", scores);
        }

        // Only Inserting a Value If the Key Has No Value
        {
            let mut scores = HashMap::new();
            scores.insert(String::from("key_a"), 100);
            scores.insert(String::from("key_b"), 200);
            scores.entry("key_b".to_string()).or_insert(1000);
            {
                let e = scores.entry("key_c".to_string());
                e.or_insert(500);
            }
            println!("scores: {:?}", scores);
            
            let e = scores.entry("key_d".to_string()).or_default();
            *e = 1000;
            println!("scores: {:?}", scores);
        }

        // Updating a Value Based on the Old Value
        {
            let t = "a s d f g h h j y e  d a as asd s s f f a sa as df a a";
            let mut m = HashMap::new();
            for w in t.split_whitespace() {
                let w_count = m.entry(w).or_insert(0);
                *w_count += 1;
                println!("    w: {w}, count: {w_count}");
            }
            println!("m: {:?}", m);
        }
    }

    // Hashing Functions
    {
        use std::collections::HashMap;
        use std::collections::hash_map::RandomState;
        use std::time::{Instant};
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let hasher = RandomState::new();
        let mut m = HashMap::with_hasher(hasher);

        let time_started = Instant::now();

        for i in 0..100_000 {
            let k_rand = rng.gen_range(0..200_000);
            m.insert(i, format!("value with {k_rand}"));
        }

        let duration = time_started.elapsed();

        println!("m: {:?}, duration: {:?}", m.len(), duration.as_millis());
    }

    // Summary
    {
        // TODO: Tasks...
    }

}
