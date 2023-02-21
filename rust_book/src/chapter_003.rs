pub fn run () {
    println!("3. Common Programming Concepts");
    chapter_3_1();
    chapter_3_2();
    chapter_3_3();
    chapter_3_4();
    chapter_3_5();
}

fn chapter_3_1 () {
    println!("3.1. Variables and Mutability");
    
    // Mutable var
    let mut x = 5;
    println!("x = {}", x);
    x = 20;
    println!("x = {}", x);
    
    // Constant
    const MAX_ID: i32 = 1024;
    println!("MAX_ID = {}", MAX_ID);

    // Shadowing
    let y = 10;
    println!("(1) y = {}", y);
    
    // Shadows a value
    let y = 100;
    println!("(2) y = {}", y);
    
    {
        // Shadows in a scope
        let y = 128;
        println!("(2.1) y = {}", y);
    }

    println!("(2) y = {}", y);

    // Shadows a type
    let y = "Type of &str";
    println!("(3) y = {}", y);

    // mut vs shadow
    let s = "12345";
    println!("s: {}", s);
    let s = s.len();
    println!("s: {}", s);

    // let mut m = "abc";
    // m = m.len(); // error: mismatched types
}

fn chapter_3_2 () {
    println!("3.2. Data Types");
    
    // Integers...
    let var_i8: i8 = i8::MAX;
    let var_i16: i16 = i16::MAX;
    let var_i32: i32 = i32::MAX;
    let var_i64: i64 = i64::MAX;
    let var_i128: i128 = i128::MAX;
    let var_isize: isize = isize::MAX;

    println!("var_i8: {}", var_i8);
    println!("var_i16: {}", var_i16);
    println!("var_i32: {}", var_i32);
    println!("var_i64: {}", var_i64);
    println!("var_i128: {}", var_i128);
    println!("var_isize: {}", var_isize);
    
    let var_u8: u8 = u8::MAX;
    let var_u16: u16 = u16::MAX;
    let var_u32: u32 = u32::MAX;
    let var_u64: u64 = u64::MAX;
    let var_u128: u128 = u128::MAX;
    let var_usize: usize = usize::MAX;

    println!("var_u8: {}", var_u8);
    println!("var_u16: {}", var_u16);
    println!("var_u32: {}", var_u32);
    println!("var_u64: {}", var_u64);
    println!("var_u128: {}", var_u128);
    println!("var_usize: {}", var_usize);

    // Integer literals...
    println!("Integer literals: {:?}", (1_024, 0xBEAF, 0o73, 0b101011001111, b'A'));

    // Integer Overflow...
    
    // let var_i8_o: i8 = 126;
    // println!("var_i8_o {}", var_i8_o);
    // let var_i8_o: i8 = var_i8_o + 2; // thread 'main' panicked at 'attempt to add with overflow', src/chapter_003.rs:96:24

    let var_i8_o: i8 = 126;
    println!("var_i8_o: {}", var_i8_o); // 126

    let var_i8_o = i8::wrapping_add(var_i8_o, 1);
    println!("var_i8_o + 1: {}", var_i8_o); // 127

    let var_i8_o = i8::wrapping_add(var_i8_o, 1);
    println!("var_i8_o + 1: {}", var_i8_o); // -128
    
    let var_i8_o: Option<i8> = i8::checked_add(126 as i8, 1 as i8);
    println!("i8::checked_add(126 as i8, 1 as i8): {:?}", var_i8_o); // Some(127)
    
    let var_i8_o: Option<i8> = i8::checked_add(127 as i8, 1 as i8);
    println!("i8::checked_add(127 as i8, 1 as i8): {:?}", var_i8_o); // None

    let var_i8_o: (i8, bool) = i8::overflowing_add(126 as i8, 1 as i8);
    println!("i8::overflowing_add(126 as i8, 1 as i8): {:?}", var_i8_o); // (127, false)
    
    let var_i8_o: (i8, bool) = i8::overflowing_add(127 as i8, 1 as i8);
    println!("i8::overflowing_add(127 as i8, 1 as i8): {:?}", var_i8_o); // (-128, true)

    let var_i8_o = i8::saturating_add(126 as i8, 1 as i8);
    println!("i8::saturating_add(126 as i8, 1 as i8): {}", var_i8_o); // 127
    
    let var_i8_o = i8::saturating_add(127 as i8, 1 as i8);
    println!("i8::saturating_add(127 as i8, 1 as i8): {}", var_i8_o); // 127

    // Floating-Point Types...
    let var_f64 = 1.0;
    let var_f32: f32 = 2.0;
    println!("{:?}", (var_f64, var_f32));
    println!("f32 range: {}..{}", f32::MIN, f32::MAX);
    println!("f64 range: {}..{}", f64::MIN, f64::MAX);
    println!("f32::MAX_10_EXP: {}", f32::MAX_10_EXP);
    println!("f32::MAX_EXP: {}", f32::MAX_EXP);

    // Numeric Operations...
    let sum = 5 + 10;
    let diff = 100.9 - 34.4;
    let prod = 4 * 90;
    let div_quotient = 32.5 / 32.8;
    let div_floored = 2 / 3;
    let remainder = 43 % 5;
    println!("{:?}", (sum, diff, prod, div_quotient, div_floored, remainder));

    // The Boolean Type...
    let var_b = true;
    let var_b2: bool = false;
    println!("{:?}", (var_b, var_b.to_string(), var_b2, var_b2.to_string()));

    // The Character Type...
    let avo1: char = '🥝';
    let avo2 = '\u{1F95D}';
    let c = 'A';
    println!("avo: {}, {}, {}", avo1, avo2, c);

    // The Tuple Type...
    let var_tup1: (i8, f64, i128) = (100, 32.32, 1024);
    println!("var_tup1: {:?}", var_tup1);

    // Tuple destructing
    let (sub1, sub2, sub3) = var_tup1;
    println!("tuple destructing: {}, {}, {}", sub1, sub2, sub3);

    // Access tuples elements
    println!("tuple access: {}, {}, {}", var_tup1.0, var_tup1.1, var_tup1.2);

    // Print tuple addresses
    println!("{:p} {:p} {:p}", &var_tup1, &var_tup1.0, &sub1);

    // Unit type and unit value
    let var_unit: () = ();
    println!("var_unit: {:?}", var_unit);

    // The Array Type...
    let var_arr1 = [1, 2, 3, 4, 5];
    println!("var_arr1: {:?} lenght {}", var_arr1, var_arr1.len());
    println!("var_arr1 mult 10: {:?}", var_arr1.map(|v| v * 10));
    let var_arr2_gen = [128; 12];
    println!("var_arr2_gen: {:?}", var_arr2_gen);
    let first = var_arr1[0];
    println!("var_arr1[0]: {0}", first);

    // Guessing array value...
    let var_arr3 = [1, 2, 3, 4, 5];
    let index = "3".to_string();
    // Enables command line user input
    // io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index
        .trim().parse().expect("Entered index is not a number");
    let element = var_arr3[index];
    println!("Index: {}, value: {}", index, element);
    for x in var_arr3 {
        println!("x: {}", x);
    }
}

fn chapter_3_3 () {
    println!("3.3. Functions");
    another_function(10);
    let x = 100;
    let y = {
        let x = x + x;
        x * x
    };
    let z = {
        let a = 200;
        a + 5
    };
    println!("x: {}, y: {}, z: {}", x, y, z);
    let r = some_add(10, 20);
    println!("r = {}, some_add returns {}", r, some_add(10, 20));
}

fn another_function (x: i32) {
    println!("x: {}", x);
}

fn some_add (x: i32, y: i32) -> i32 {
    x + y
}

fn chapter_3_4 () {
    println!("3.4. Comments");
    // Some comment
    // Another comment
    /*
    Also comment
    */
}

fn chapter_3_5 () {
    println!("3.5. Control Flow");
    let number = 51;
    if number > 3 {
        println!("True");
    } else {
        println!("False");
    }

    // if else if else ...
    if number % 4 == 0{
        println!("div by 4");
    } else if number % 3 == 0 {
        println!("div by 3");
    } else {
        println!("number {}", number);
    }

    // Ternary operator
    let result = if number == 51 { "Number is 51" } else { "False" };
    println!("result: {}", result);

    // Repeating code with loop
    let mut i = 0;
    const LOOP_LIMIT: i32 = 5;
    'top_level_loop: loop {
        println!("Iteration {}", i);
        i += 1;
        if i > LOOP_LIMIT {
            break;
        }

        let mut inner_i = 0;
        loop {
            println!(" Inner Iteration {}", inner_i);
            inner_i += 1;
            if inner_i == 5 {
                break;
            }

            if inner_i == 3 && i == 3 {
                println!("  Break top level loop");
                break 'top_level_loop;
            }
        }
    }

    // Returning values from loop
    let mut i = 0;
    let mut accum = 0;
    let loop_result = loop {
        i += 1;
        accum += i;
        if i == LOOP_LIMIT {
            break accum;
        }
    };
    println!("Sum from 1 to {} is {}", LOOP_LIMIT, loop_result);

    // Conditional loops with while
    let mut i = 0;
    while i < LOOP_LIMIT {
        println!("i = {}", i);
        i += 1;
    }

    // Looping throught a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut i = 0;
    while i < LOOP_LIMIT {
        println!("a[{}]: {}", i, a[i as usize]);
        i += 1;
    }
    
    // Loop throught an array
    for e in a {
        println!("for range a[i]: {}", e);
    }

    // Loop throught an a reversed sequence is ints
    for i in (0..a.len()).rev() {
        println!("for range rev a[{}]: {}", i, a[i]);
    }
}
