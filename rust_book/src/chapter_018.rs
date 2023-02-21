pub fn run() {
    println!("18. Patterns and Matching");
    chapter_018_1();
    chapter_018_2();
    chapter_018_3();
}

fn chapter_018_1() {
    println!("18.1. All the Places Patterns Can Be Used");

    // match Arms
    {
        let x = match Some(10) {
            None => None,
            Some(i) => Some(i + 5),
        };
        println!("x = {:?}", x);
    }

    // Conditional if let Expressions
    {
        let favorite_color: Option<&str> = None;
        let is_monday = false;
        let age: Result<u8, _> = "35".parse();

        if let Some(color) = favorite_color {
            println!("Favorite color = {:?}", color);
        } else if is_monday {
            println!("Is a monday");
        } else if let Ok(age) = age {
            println!("Age = {}", age);
            if age > 30 {
                println!("You are young! = {age}");
            } else {
                println!("You are old! = {age}");
            }
        } else {
            println!("Neither from the above");
        }
    }

    // while let Conditional Loops
    {
        let mut stack = Vec::new();
        stack.push(10);
        stack.push(20);
        stack.push(30);

        while let Some(top) = stack.pop() {
            println!("top = {:?}", top);
        }

        println!("stack = {:?}", stack);
        assert_eq!(stack.len(), 0);
    }

    // for Loops
    {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8,];
        for (index, value) in v.iter().enumerate() {
            println!("index = {}, value = {}", index, value);
        }
    }

    // let Statements
    {
        let (a, b, c) = (10, 20, 30);
        let (x, y, z) = (1.2, "2", 3 as usize);
        println!("a = {}, b = {}, c = {}", a, b, c);
        println!("x = {}, y = {}, z = {}", x, y, z);
    }

    // Function Parameters
    {
        fn print_coordinates(&(x, y): &(f64, f64)) {
            println!("x = {}, y = {}", x, y);
        }

        let c = (2.2, 4.4);
        print_coordinates(&c);
        
        // Using a closure
        let c_pc = |&(x, y): &(i32, i32)| {
            println!("c_pc :: x = {}, y = {}", x, y);
        };
        let c = (100, 105);
        c_pc(&c);
    }
}

fn chapter_018_2() {
    println!("18.2. Refutability: Whether a Pattern Might Fail to Match");

    // 1
    let _o_val: Option<i32> = Some(10);
    let o_val: Option<i32> = None;

    // Error
    // refutable pattern in local binding: `None` not covered
    // let Some(y) = x;

    let x = if let Some(sub_x) = o_val {
        println!("sub_x = {sub_x}");
        sub_x
    } else { 
        -1
    };
    println!("x = {x}");

    // 2
    let a =  if x >= 10 {
        println!("x = {x}");
        20
    } else {
        -10
    };
    println!("a = {a}");

    // 3 
    if let Some(b) = o_val {
        println!("b = {b}");
    }

    // 4
    // Always valid code - warning without the allow macro below:
    #[allow(irrefutable_let_patterns)]
    if let x = 5 {
        println!("{}", x);
    };
}

fn chapter_018_3() {
    println!("18.3. Pattern Syntax");

    // Matching Literals
    {
        let x = 2;
        match x {
            1 => println!("x = 1"),
            2 => println!("x = 2"),
            3 => println!("x = 3"),
            _ => println!("_ => x = {x}"),
        }

        let o_0 = Some(0);
        let o_3 = Some(3);
        let o_none: Option<i32> = None;

        fn option_to_string(o: Option<i32>) -> String {
            match o {
                Some(1) => "Some(1)".to_owned(),
                Some(3) => "Some(three)".to_owned(),
                Some(_) => "Some(_)".to_owned(),
                None => "None".to_owned(),

                // Alternative way to handle all other cases
                // _ => "_ => _ anything".to_owned(),
            }
        }

        let o_0_s = option_to_string(o_0);
        println!("o_0: {:?}", o_0_s);
        let o_3_s = option_to_string(o_3);
        println!("o_3: {:?}", o_3_s);
        let o_none_s = option_to_string(o_none);
        println!("o_None: {:?}", o_none_s);
    }

    // Matching Named Variables
    {
        let _x = Some(5);
        let x = None;
        let y = 10;
    
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {:?}", x),
        }
    
        println!("at the end: x = {:?}, y = {y}", x);        
    }

    // Multiple Patterns
    {
        let i = 100;

        match i {
            1 => println!("i = 1"),
            2 | 100 => println!("2 or 100"),
            _ => println!("_ => i = {i}"),
        }
    }

    // Matching Ranges of Values with ..=
    {
        fn print_i(i: i32) {
            match i {
                1..=24 => println!("i = 1..=24"),
                25..=50 => println!("i = 25..50"),
                _ => println!("_ => other i = {i}"),
            }
        }
        print_i(10);
        print_i(30);
        print_i(300);

        fn char_to_type(c: char) -> &'static str {
            match c {
                'a'..='z' => "lowercase",
                'A'..='Z' => "uppercase",
                '0'..='9' => "number",
                '!'..='+' => "punctuation",
                        _ => "other",
            }
        }

        let ca_s = char_to_type('a');
        println!("ca_s: {ca_s:?}");
        let c4_s = char_to_type('4');
        println!("c4_s: {c4_s:?}");
        let ca_s = char_to_type('A');
        println!("ca_s: {ca_s:?}");
        let c_underscore_s = char_to_type('_');
        println!("c_underscore_s: {c_underscore_s:?}");
        let c_hit_s = char_to_type('!');
        println!("c_hit_s: {c_hit_s:?}");
    }

    // Destructuring Structs
    {
        use crate::common::chapter_018::Point;

        let p = Point::new(6500, 200_000_000_000);
        
        let Point { 
            x: destructed_x,
            y: destructed_y,
        } = p;

        let Point { x, y } = p;

        println!("p = {:?}", p);
        println!("destructed_x = {:?}", destructed_x);
        println!("destructed_y = {:?}", destructed_y);
        println!("x = {:?}", x);
        println!("y = {:?}", y);

        
        let p = Point::new(1, 2);
        match p {
            Point { x: 6500, y: _ } => println!("Matching x only = {x}"),
            Point { x, y: 200_000_000_000 } => println!("Matching both {x} and {y}"),
            Point { x, y } => println!("Matching anything, p = {:?}", (x, y)),
        }

        let p = Point::new(6500, 6501);
        match p {
            Point { x: 6500, y: _ } => println!("Matching x only = {x}"),
            Point { x, y: 200_000_000_000 } => println!("Matching both {x} and {y}"),
            Point { x, y } => println!("Matching anything, p = {:?}", (x, y)),
        }
        
        let p = Point::new(6501, 200_000_000_000);
        match p {
            Point { x: 6500, y: _ } => println!("Matching x only = {x}"),
            Point { x, y: 200_000_000_000 } => println!("Matching both {x} and {y}"),
            Point { x, y } => println!("Matching anything, p = {:?}", (x, y)),
        }
    }

    // Destructuring Enums
    {
        use crate::common::chapter_006::Message;

        fn message_to_string(m: Message) -> String {
            match m {
                Message::Quit =>
                    "enum: Message::Quit".to_string(),
                Message::Move { x, y } => 
                    format!("struct: Message::Move: {{ x: {x}, y: {y} }},").to_string(),
                Message::Write(s) =>
                    format!("string: Message::Write({s})").to_string(),
                Message::Color(r, g, b) =>
                    format!("tuple: Message::Color({r}, {g}, {b})").to_string(),
            }
        }
        let s = message_to_string(Message::Quit);
        println!("s = {:?}", s);

        let s = message_to_string(Message::Move { x: 1, y: 2 });
        println!("s = {:?}", s);

        let s = message_to_string(Message::Write("Hello".to_string()));
        println!("s = {:?}", s);

        let s = message_to_string(Message::Color(1, 2, 3));
        println!("s = {:?}", s);
    }

    // Destructuring Nested Structs and Enums
    {
        use crate::common::chapter_006::{MessageV2, Color};

        let mv2_1 = MessageV2::ChangeColor(Color::Rgb(1, 1, 1));
        let mv2_2 = MessageV2::ChangeColor(Color::Hsv(10, 10, 10));

        fn message_v2_to_color_tuple(mv2: MessageV2) -> (i32, i32, i32) {
            match mv2 {
                MessageV2::ChangeColor(Color::Rgb(r, g, b)) => (r, g, b),
                MessageV2::ChangeColor(Color::Hsv(h, s, v)) => (h, s, v),
                _ => (0, 0, 0),
            }
        }

        let (c1, c2, c3) = message_v2_to_color_tuple(mv2_1);
        println!("c1, c2, c3 = {} {} {}", c1, c2, c3);

        let (c1, c2, c3) = message_v2_to_color_tuple(mv2_2);
        println!("c1, c2, c3 = {} {} {}", c1, c2, c3);
    }

    // Destructuring Structs and Tuples
    {
        use crate::common::chapter_006::Message;
        use crate::common::chapter_018::Point;
        fn get_something_complex() -> (i32, i32, isize, isize) {
            let (xx, yy, x, y) = 
                if let (
                    Message::Move { x: xx, y: yy }, Point { x, y }
                ) = (
                    Message::Move { x: 100, y: 200 },
                    Point::new(1, 2)
                ) { 
                    (xx, yy, x, y) 
                } else {
                    (0, 0, 0, 0)
                };
            (xx, yy, x, y)
        }
        let c = get_something_complex();
        println!("c = {:?}", c);
    }

    // Ignoring an Entire Value with _
    {
        fn ignore_input_arg(i: i32, _: i32) {
            println!("i = {}", i);
        }
        ignore_input_arg(1, 2);
    }

    // Ignoring Parts of a Value with a Nested _
    {
        let n = (5, 10, 15, 20, 25);
        match n {
            (_, s, _, _, f) => {
                println!("second part = {:?}", s);
                println!("fifth part = {:?}", f);
            },
        }

        let vo = vec![
            (None, None, None),
            (None, None, Some(10)),
            (Some(10), None, None),
            (None, Some(10), None),
            (Some(10), Some(10), Some(10)),
        ];

        fn test_tuple_of_something(t: (Option<i32>, Option<i32>, Option<i32>)) {
            let (o1, o2, o3) = t;
            println!("t = {t:?}");
            match (o1, o2, o3) {
                (_, Some(_), Some(o3_v)) => {
                    println!("    Ignore any 1, 2 is any Some, o3_v = {:?}", o3_v);
                },
                (.., Some(o3_v)) => {
                    println!("    Ignore any 1 and 2, o3_v = {:?}", o3_v);
                },
                (.., None) => println!("    Ignore any 1 and 2, o3 is None"),
            }
        }

        for o in vo {
            test_tuple_of_something(o);
        }
    }

    // Ignoring an Unused Variable by Starting Its Name with _
    {
        let s = Some(String::from("test"));
        // _s takes ownership of s
        if let Some(_s) = s {
            println!("s = {:?}", _s);
        }
        
        let s = Some(String::from("test"));
        // The owner of s still the same
        if let Some(_) = s {
            println!("s = {:?}", s);
        }
    }

    // Ignoring Remaining Parts of a Value with ..
    {
        use crate::common::chapter_018::Point4;

        let p4 = Point4::new(10, 20, 30, 40);

        match p4 {
            // Point4 { x, y: _, z: _, w: _, .. } => {
            Point4 { x, .. } => {
                println!("x = {:?}", x);
            }
        }

        let numbers = (2, 4, 8, 16, 32, 64, 128);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }

        // // Error: `..` can only be used once per tuple pattern
        // match numbers {
        //     (first, .., middle, .., last) => {
        //         println!("Some numbers: {first}, {last}");
        //     }
        // }
    }

    // Extra Conditionals with Match Guards
    {
        let vec_s = vec![
            Some(String::from("test")),
            Some(String::from("abc")),
            Some(String::from("ABC")),
        ];

        fn test_match_guard(s: Option<String>) {
            match s {
                Some(s) if s == "test" => {
                    println!("TEST: s = {:?}", s);
                },
                Some(s) if s == "abc" => {
                    println!("ABC: s = {:?}", s);
                },
                Some(s) => {
                    println!("Other string: {}", s);
                },
                None => {
                    println!("None");
                }
            }
        }

        for s in vec_s {
            test_match_guard(s);
        }

        let i = 25;
        match i {
            0..=30 if i == 25 => println!("i = 25"),
            0..=30            => println!("0..=30"),
            _                 => println!("i = {}", i),
        }
    }

    // @ Bindings
    {
        let i = 25;
        match i {
            i_variable @ 0..=30 => {
                println!("i = {}, i_variable = {}", i, i_variable);
            }
            31..=50 => println!("31..=50"),
            _       => println!("i = {}", i),
        }

        #[derive(Debug)]
        enum Command {
            Open {
                id: i32,
                name: String,
            }
        }

        let c = Command::Open {
            id: 100,
            name: "test".to_string(),
        };

        match c {
            Command::Open {
                // Any value in the range between 0 and 49, no variable provided the get particaler value 
                id: 0..=49,
                name 
            } => println!("name = {}", name),
            Command::Open { 
                // Any value in the range between 50 and 100, specific id provided the get particaler value
                id: specific_id @  50..= 100,
                name,
            } => println!("id = {}, name = {}", specific_id, name),
            Command::Open {
                // Any posible i32 value
                id, name
            } => println!("Anything else, id = {}, name = {}", id, name),
            // _ => println!("id = {:?}", c),
        }
    }
}
