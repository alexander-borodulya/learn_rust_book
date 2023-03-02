pub fn run(_subchapter_index: u32) {
    println!("10. Generic Types, Traits, and Lifetimes");
    chapter_010_0();
    chapter_010_1();
    chapter_010_2();
    chapter_010_3();
}

fn chapter_010_0() {
    println!("10.0 - Generic Types, Traits, and Lifetimes");
    // 1 - Inline, with two variants
    {
        // A
        let number_list = vec![1, 2, 3, 4, 5];
        let mut largest = number_list[0];
        for n in number_list {
            if n > largest {
                largest = n;
            }
        }
        println!("(1) largest, a: {}", largest);

        // B
        let number_list = vec![1, 2, 3, 4, 5, 100, 200, 300, 400, 500];
        let mut largest = number_list[0];
        for n in number_list {
            if n > largest {
                largest = n;
            }
        }
        println!("(1) largest, b: {}", largest);
    }

    // 2- Extract into the function
    {
        fn find_largest(list: &[i32]) -> i32 {
            let mut largest = list[0];
            for &i in list {
                if i > largest {
                    largest = i;
                }
            }
            largest
        }

        // A
        let number_list = vec![1, 2, 3, 4, 5];
        let largest = find_largest(&number_list);
        println!("(2) largest, a: {}", largest);

        // B
        let number_list = vec![1, 2, 3, 4, 5, 100, 200, 300, 400, 500];
        let largest = find_largest(&number_list);
        println!("(2) largest, b: {}", largest);
    }
}

fn chapter_010_1() {
    println!("10.1. Generic Data Types");

    // In Function Definition
    {
        fn find_largest_i32(list: &[i32]) -> i32 {
            let mut largest = list[0];
            for &i in list {
                if i > largest {
                    largest = i;
                }
            }
            largest
        }

        fn find_largest_char(list: &[char]) -> char {
            let mut largest = list[0];
            for &i in list {
                if i > largest {
                    largest = i;
                }
            }
            largest
        }

        let arr_i = vec![10, 20, 30, 400, 500];
        let arr_c = vec!['a', 'b', 'c', 'd', 'x', 'y'];
        println!("largest i: {:?}", find_largest_i32(&arr_i));
        println!("largest c: {:?}", find_largest_char(&arr_c));
    }

    // Make it A Generic Function
    {
        // Calling this...
        //
        // fn find_largest<T>(list: &[T]) -> T {
        //     let mut largest = list[0];
        //     for &i in list {
        //         if i > largest {
        //             largest = i;
        //         }
        //     }
        //     largest
        // }
        //
        // Generates the following output...
        //
        // error[E0369]: binary operation `>` cannot be applied to type `T`
        //   --> src\chapter_010.rs:94:22
        //    |
        // 94 |                 if i > largest {
        //    |                    - ^ ------- T
        //    |                    |
        //    |                    T
        //    |
        // help: consider restricting type parameter `T`
        //    |
        // 91 |         fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
        //    |                          ++++++++++++++++++++++
        //
        // For more information about this error, try `rustc --explain E0369`.
        // error: could not compile `learn-rust-book` due to previous error

        // TODO: Fix Later...
    }

    // In Struct Definitions
    {
        // Single Type
        #[allow(dead_code)]
        {
            #[derive(Debug)]
            struct Point<T> {
                x: T,
                y: T,
            }
    
            let p_int = Point { x: 0, y: 1 };
            let p_float = Point { x: 1.0, y: 5.0 };
            
            // Won't compile - Mismatched types int vs float
            // let p_int_float = Point { x: 0, y: 1.0 };
    
            println!("p_int: {:?}", p_int);
            println!("p_float: {:?}", p_float);
        }

        // Multiple Types
        #[allow(dead_code)]
        {
            #[derive(Debug)]
            struct Point<T, U> {
                x: T,
                y: U,
            }
    
            let p_int = Point { x: 0, y: 1 };
            let p_float = Point { x: 1.0, y: 5.0 };
            let p_int_float = Point { x: 0, y: 1.0 }; // OK
            let p_weird = Point { x: -100, y: "+200".to_string() };
    
            println!("p_int: {:?}", p_int);
            println!("p_float: {:?}", p_float);
            println!("p_int_float: {:?}", p_int_float);
            println!("p_weird: {:?}", p_weird);
        }
    }

    // In Method Definitions
    {
        struct Point <T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        // Another specialization
        impl Point<usize> {
            fn usize_only_info(&self) -> String {
                "impl Point<usize>::usize_only_info(&self) -> String".to_string()
            }
        }

        let p = Point { x: 20, y: 1 };
        println!("P.x: {:?}", p.x());

        let p_1_f32 = Point { x: 2.2, y: 3.3 };
        let p_2_f32: Point<f32> = Point { x: 2.2, y: 3.3 };
        println!("p_1_f32 {:?}", p_1_f32.distance_from_origin());
        println!("p_2_f32 {:?}", p_2_f32.distance_from_origin());

        let p_usize = Point { x: 0_usize, y: 1_usize };
        println!("p_usize: {:?}", p_usize.usize_only_info());

        // Mixup Example - Split on two different versions
        {
            // Same Type Mixup
            {
                #[derive(Debug)]
                struct Point<X1, Y1> {
                    x: X1,
                    y: Y1,
                }

                impl <X1, Y1> Point<X1, Y1> {
                    fn mixup (self, other: Point<X1, Y1>) -> Point<X1, Y1> {
                        Point { 
                            x: self.x, 
                            y: other.y
                        }
                    }
                }

                let p_1 = Point { x: 0, y: 1 };
                let p_2 = Point { x: 2, y: 3 };
                println!("Mixup 1: p_1: {:?}, p_2: {:?}", p_1, p_2);
                
                let p_3 = p_1.mixup(p_2);
                println!("Mixup 1: p_3: {:?}", p_3);
            }
            
            // Different Type Mixup
            {
                #[derive(Debug)]
                struct Point<X1, Y1> {
                    x: X1,
                    y: Y1,
                }

                impl <X1, Y1> Point<X1, Y1> {
                    fn mixup <X2, Y2> (self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                        Point { 
                            x: self.x, 
                            y: other.y
                        }
                    }
                }

                let p_1 = Point { x: 'A', y: 1.0 };
                let p_2 = Point { x: 100 , y: "String".to_string() };
                println!("Mixup 2: p_1: {:?}, p_2: {:?}", p_1, p_2);
    
                let p_3 = p_1.mixup(p_2);
                println!("Mixup 2: p_3: {:?}", p_3);
            }
        }
    }

    // Another different generic arguments
    #[allow(dead_code)]
    {
        #[derive(Debug)]
        struct MyObject <T> {
            value: T,
        }

        impl <T> MyObject <T>
        where
            T: std::fmt::Debug,
        {
            fn new(value: T) -> Self {
                println!("MyObject::new, T={:?}, value={:?}", std::any::type_name::<T>(), value);
                Self { value }
            }
        }

        // impl <f64: std::fmt::Debug> MyObject <f64> {
        #[allow(non_camel_case_types)]
        #[allow(clippy::builtin_type_shadow)]
        impl <f64> MyObject <f64>
        where
            f64: std::fmt::Debug
        {
            fn new_with_f64(value: f64) -> Self {
                println!("MyObject::new, impl <f64> MyObject <f64>");
                MyObject::new(value)
            }
        }

        let mo_1 = MyObject::new("string value");
        let mo_2 = MyObject::new_with_f64(13.14);

        println!("mo_1: {:?}, mo_2: {:?}", mo_1, mo_2)
    }

    // Performance of Code Using Generics
    #[allow(dead_code, non_camel_case_types)]
    {
        #[derive(Debug)]
        enum MyOption<T> {
            Some(T),
            None,
        }

        let i = MyOption::Some(10);
        let f = MyOption::Some(1.0);

        #[derive(Debug)]
        enum MyOption_i32 {
            Some(i32),
            None,
        }

        #[derive(Debug)]
        enum MyOption_f64 {
            Some(f64),
            None,
        }

        let ii = MyOption_i32::Some(100);
        let ff = MyOption_f64::Some(100.100);

        println!("{:?}, {:?}", i, f);
        println!("{:?}, {:?}", ii, ff);
    }
}

fn chapter_010_2() {
    println!("10.2. Traits: Defining Shared Behavior");

    // Defining a Trait
    {
        // Summary - Common Trait
        pub trait Summary {
            fn summarize(&self) -> String;
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        // Usage...
        {
            let tweet = Tweet {
                username: String::from("UserName"),
                content: String::from("A stream is an asynchronous series of values. It is the asynchronous equivalent to Rust's std::iter::Iterator and is represented by the Stream trait."),
                reply: false,
                retweet: false,
            };
            println!("1 new tweet: {}", tweet.summarize());
        }
    }

    // Default Implemetations
    #[allow(dead_code, unused_variables)]
    {
        // Todo: Move Structs into Module
        pub trait Summary {
            fn summarize_author(&self) -> String;
            fn summarize(&self) -> String {
                format!("(Read more from {:?}...)", self.summarize_author())
            }
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        // Default implemetation usage: specify empty impl block
        impl Summary for NewsArticle {
            fn summarize_author(&self) -> String {
                format!("author {}", self.author)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }            
        }
        
        // Usage...
        {
            let article = NewsArticle {
                headline: String::from("Headline Text"),
                location: String::from("Location Text"),
                author: String::from("Author Text"),
                content: String::from("Content Text"),
            };

            let tweet = Tweet {
                username: String::from("UserName"),
                content: String::from("A stream is an asynchronous series of values. It is the asynchronous equivalent to Rust's std::iter::Iterator and is represented by the Stream trait."),
                reply: false,
                retweet: false,
            };

            println!("1 new tweet: {}", tweet.summarize());
            println!("Article with default summarize: {}", article.summarize());
        }

        // Traits as Parameters
        {
            pub struct StackOverflow {}
            
            impl StackOverflow {
                pub fn summarize(&self) -> String {
                    "Stack Overflow Anwers are Boring...".to_string()
                }
            }

            pub fn notify(item: &impl Summary) {
                println!("Breaking: {}", item.summarize())
            }

            let article = NewsArticle {
                headline: String::from("Headline Text"),
                location: String::from("Location Text"),
                author: String::from("Author Text"),
                content: String::from("Content Text"),
            };

            let tweet = Tweet {
                username: String::from("UserName"),
                content: String::from("A stream is an asynchronous series of values. It is the asynchronous equivalent to Rust's std::iter::Iterator and is represented by the Stream trait."),
                reply: false,
                retweet: false,
            };

            let so_answer = StackOverflow {};

            notify(&article);
            notify(&tweet);
            so_answer.summarize();

            //
            // Calling this...
            //
            //notify(&so_answer);
            // 
            // Generates the following output:
            //
            // error[E0277]: the trait bound `StackOverflow: chapter_010_2::Summary` is not satisfied
            //    --> src\chapter_010.rs:432:20
            //     |
            // 432 |             notify(&so_answer);
            //     |             ------ ^^^^^^^^^^ the trait `chapter_010_2::Summary` is not implemented for `StackOverflow`
            //     |             |
            //     |             required by a bound introduced by this call
            //     |
            //     = help: the following other types implement trait `chapter_010_2::Summary`:
            //               chapter_010_2::NewsArticle
            //               chapter_010_2::Tweet
            // note: required by a bound in `notify`
            //    --> src\chapter_010.rs:408:39
            //     |
            // 408 |             pub fn notify(item: &impl Summary) {
            //     |                                       ^^^^^^^ required by this bound in `notify`

            // For more information about this error, try `rustc --explain E0277`.
            // error: could not compile `learn-rust-book` due to previous error
        }

        // Trait Bound Syntax
        {
            pub fn notify<T: Summary>(item: &T) {
                println!("notify (with bounds): {}", item.summarize())
            }

            pub fn notify_same<T: Summary>(a: &T, b: &T) {
                println!("notify_same (with bounds): a: {}", a.summarize());
                println!("notify_same (with bounds): b: {}", b.summarize());
            }
            
            pub fn notify_all(a: &impl Summary, b: &impl Summary) {
                println!("notify_all (with bounds): a: {}", a.summarize());
                println!("notify_all (with bounds): b: {}", b.summarize());
            }

            let article = NewsArticle {
                headline: String::from("Headline Text 2 "),
                location: String::from("Location Text 2"),
                author: String::from("Author Text 2"),
                content: String::from("Content Text 2"),
            };

            let tweet_2 = Tweet {
                username: String::from("UserName 2"),
                content: String::from("2 A stream is an asynchronous series of values. It is the asynchronous equivalent to Rust's std::iter::Iterator and is represented by the Stream trait."),
                reply: false,
                retweet: false,
            };

            let tweet_3 = Tweet {
                username: String::from("UserName 3"),
                content: String::from("3 A stream is an asynchronous series of values. It is the asynchronous equivalent to Rust's std::iter::Iterator and is represented by the Stream trait."),
                reply: false,
                retweet: false,
            };

            notify(&article);
            notify(&tweet_2);
            notify_all(&article, &tweet_3);
            notify_same(&tweet_2, &tweet_3);
        }

        // Simplifying Multiple Trait Bound with + Syntax
        {
            struct Object {
                id: String,
            }

            impl Summary for Object {
                fn summarize_author(&self) -> String {
                    format!("author id: {}", self.id)
                }
            }

            impl std::fmt::Display for Object {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "DISPLAING OBJECT: {}", self.summarize())
                }
            }

            pub fn notify_1(item: &(impl Summary + std::fmt::Display)) {
                println!("notify_1: {}", item);
            }

            pub fn notify_2<T: Summary + std::fmt::Display>(item: &T) {
                println!("notify_2: {}", item);
            }

            let obj_1 = Object { id: "awesome-id".to_string() };
            notify_1(&obj_1);
            notify_2(&obj_1);

            // Clearer Trait Bounds with <where> Clauses
            {
                pub fn notify_3 <T> (item: &T)
                    where T: Summary + std::fmt::Display
                {
                    println!("notify_3: {}", item);
                }

                let obj_2 = Object { id: "awesome-id-obj_2".to_string() };

                notify_3(&obj_2);
                notify_3(&Object { id: "awesome-id-obj_tmp".to_string() });
            }
        }

        // Clearer Trait Bounds with <where> Clauses
        {
            pub fn some_function <T: std::fmt::Display + Clone, U: Clone + std::fmt::Debug> (t_val: &T, u_val: &U) -> i32 {
                println!("Both satisfied: {} and {:?}", t_val, u_val);
                128
            }
            pub fn some_function_too <T: std::fmt::Display + Clone, U: Clone + std::fmt::Debug> (t_val: &T, u_val: &U) -> i32 
                where T: std::fmt::Display + Clone,
                      U: Clone + std::fmt::Debug
            {
                println!("Both satisfied to: {} and {:?}", t_val, u_val);
                256
            }

            let i = 32;
            let f = 2.3;
            some_function(&i, &f);
            some_function_too(&i, &f);
        }

        // Returning Types that Implements Traits
        {
            fn return_summarizable() -> impl Summary {
                Tweet {
                    content: "content from summarizable".to_string(),
                    username: "username from summarizable".to_string(),
                    reply: true,
                    retweet: false,
                }
            }
            let t = return_summarizable();
            println!("return_summarizable: {}", t.summarize());

            // Would not work for different types that both implements the same Trait
            {
                // Calling this...
                //
                // fn return_summarizable_cond(switch: bool) -> impl Summary {
                //     if switch {
                //         NewsArticle {
                //             headline: String::from("Headline from return_summarizable_cond"),
                //             location: String::from("Location from return_summarizable_cond"),
                //             author: String::from("Author from return_summarizable_cond"),
                //             content: String::from("Content from return_summarizable_cond"),
                //         }
                //     } else {
                //         Tweet {
                //             content: "content from return_summarizable_cond".to_string(),
                //             username: "username from return_summarizable_cond".to_string(),
                //             reply: true,
                //             retweet: false,
                //         }
                //     }    
                // }
                // 
                // Generates the following output:
                // 
                // error[E0308]: `if` and `else` have incompatible types
                //    --> src\chapter_010.rs:593:25
                //     |
                // 585 |   /                     if switch {
                // 586 |   |                         NewsArticle {
                //     |  _|_________________________-
                // 587 | | |                             headline: String::from("Headline from return_summarizable_cond"),
                // 588 | | |                             location: String::from("Location from return_summarizable_cond"),
                // 589 | | |                             author: String::from("Author from return_summarizable_cond"),
                // 590 | | |                             content: String::from("Content from return_summarizable_cond"),
                // 591 | | |                         }
                //     | |_|_________________________- expected because of this
                // 592 |   |                     } else {
                // 593 | / |                         Tweet {
                // 594 | | |                             content: "content from return_summarizable_cond".to_string(),
                // 595 | | |                             username: "username from return_summarizable_cond".to_string(),
                // 596 | | |                             reply: true,
                // 597 | | |                             retweet: false,
                // 598 | | |                         }
                //     | |_|_________________________^ expected struct `chapter_010_2::NewsArticle`, found struct `chapter_010_2::Tweet`
                // 599 |   |                     }
                //     |   |_____________________- `if` and `else` have incompatible types

                // For more information about this error, try `rustc --explain E0308`.
                // error: could not compile `learn-rust-book` due to previous error
            }
        }
    }

    // Fixing the largest Function with Trait Bounds
    {
        fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];
            for &i in list {
                if i > largest {
                    largest = i;
                }
            }
            largest
        }

        let arr_i = vec![10, 20, 30, 400, 500];
        let arr_c = vec!['a', 'b', 'c', 'd', 'x', 'y'];
        println!("Fixed: largest i: {:?}", find_largest(&arr_i));
        println!("Fixed: largest c: {:?}", find_largest(&arr_c));
    }

    // Fixing: Version without Copy or Clone...
    {
        fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
            let mut largest_index = 0;
            let mut current_index = 0;

            // Allowed on demo purpose
            #[allow(clippy::explicit_counter_loop)]
            for i in list {
                if i > &list[largest_index] {
                    largest_index = current_index;
                }
                current_index += 1;
            }

            &list[largest_index]
        }

        let arr_i = vec![10, 20, 30, 400, 500];
        let arr_c = vec!['a', 'b', 'c', 'd', 'x', 'y'];
        println!("Fixed: No Copy: largest i: {:?}", find_largest(&arr_i));
        println!("Fixed: No Copy: largest c: {:?}", find_largest(&arr_c));
    }

    // Using Trait Bounds to Conditionaly Implement Methods
    {
        use std::fmt::Display;

        struct Pair<T> {
            x: T,
            y: T,
        }

        impl <T> Pair<T> {
            // fn new (x: T, y: T) -> Pair<T> {
            fn new (x: T, y: T) -> Self {
                Self { x,  y }
            }
        }

        impl <T> Pair<T> 
            where T: Display + PartialOrd
        {
            fn compare_and_display(&self) {
                if self.x >= self.y {
                    println!("Largest: x: {}", self.x);
                } else {
                    println!("Largest: y: {}", self.y);
                }
            }
        }

        let p1 = Pair::new(1, 2);
        let p2 = Pair::new(2, 1);
        let p3 = Pair::new(3, 3);

        p1.compare_and_display();
        p2.compare_and_display();
        p3.compare_and_display();

        // Conditionaly implemented ToString trait for Pair<T>
        impl <T: Display> ToString for Pair<T> {
            fn to_string(&self) -> String {
                format!("Pair {{ x: {}, y: {} }}", self.x, self.y)
            }
        }

        let p4 = Pair::new(100, 100);
        println!("p4: {}", p4.to_string());
    }

    // Playing with Traits
    {
        pub trait MyTrait {
            fn required_method(&self);
            fn optional_method(&self) {
                println!("This is optional");
            }
            fn optional_calls_required(&self) {
                self.required_method();
            }
        }

        #[derive(Debug)]
        struct MyObject {}

        impl ToString for MyObject {
            fn to_string(&self) -> String {
                format!("MyObject ({self:p})").as_str().to_string()
            }
        }

        impl MyObject {
            fn new() -> Self {
                Self { }
            }

            fn my_method(&self) {
                println!("This is my method for MyObject");

            }
        }

        impl MyTrait for MyObject {
            fn required_method(&self) {
                println!("This is required in impl MyTrait for MyObject");
            }

            fn optional_method(&self) {
                println!("This is optional in impl MyTrait for MyObject");
            }
        }

        let mo_1 = MyObject::new();
        mo_1.my_method();
        mo_1.required_method();
        mo_1.optional_method();
        mo_1.optional_calls_required();

        // Local trair for external type
        impl MyTrait for i32 {
            fn required_method(&self) {
                println!("This is required");
            }
        }
        let i = 100;
        i.required_method();
        println!("i: {:?}", i);

        // Defining a trait that has a consume method that captures an ownership
        pub trait Consumer {
            fn consume(self);
        }

        impl Consumer for MyObject {
            fn consume(self) {
                println!("Consumed: {:?}", self);
            }            
        }

        mo_1.consume();

        // The code:
        //
        // mo_1.my_method();
        //
        // Generates the following error:
        //
        // error[E0382]: borrow of moved value: `mo_1`
        //    --> src/chapter_010.rs:814:9
        //    |
        // 793 |         let mo_1 = MyObject::new();
        //    |             ---- move occurs because `mo_1` has type `chapter_010_2::MyObject`, which does not implement the `Copy` trait
        // ...
        // 812 |         mo_1.consume();
        //    |              --------- `mo_1` moved due to this method call
        // 813 |         
        // 814 |         mo_1.my_method();
        //    |         ^^^^^^^^^^^^^^^^ value borrowed here after move
        //    |
        // note: this function takes ownership of the receiver `self`, which moves `mo_1`
        //   --> src/chapter_010.rs:803:24
        //    |
        // 803 |             fn consume(self);

        #[derive(Debug, Clone, Copy)]
        struct MyObjectWithCopy {}
        impl MyObjectWithCopy { 
            fn new() -> Self { Self {} }
            fn my_method(&self) { 
                println!("MyObjectWithCopy::my_method: self: {:p}", &self); 
            }
        }
        impl Consumer for MyObjectWithCopy { 
            fn consume(self) { 
                println!("Consumed: {:?}:{:p}", self, &self); 
            } 
        }
        let mo_c_1 = MyObjectWithCopy::new();
        mo_c_1.consume();
        mo_c_1.my_method();

        // Retrurning types that implements trait
        #[allow(dead_code)]
        {
            #[derive(Debug)]
            struct MyObjectB {}
    
            impl MyObjectB {
                fn new() -> Self {
                    Self { }
                }
    
                fn my_method(&self) {
                    println!("This is my method for MyObjectB");
    
                }
            }
    
            impl MyTrait for MyObjectB {
                fn required_method(&self) {
                    println!("This is required in impl MyTrait for MyObjectB");
                }
    
                fn optional_method(&self) {
                    println!("This is optional in impl MyTrait for MyObjectB");
                }
            }

            fn get_my_trait_1() -> impl MyTrait {
                MyObject::new()
            }

            fn get_my_trait_2() -> impl MyTrait {
                MyObjectB::new()
            }

            //
            // The code:
            // //
            // fn get_my_trait_3(arg: i32) -> impl MyTrait {
            //     if arg > 100 {
            //         get_my_trait_1()
            //     } else {
            //         get_my_trait_2()
            //     }
            // }
            // 
            // Generates the following error message:
            //
            // error[E0308]: `if` and `else` have incompatible types
            // --> src/chapter_010.rs:899:21
            //     |
            // 885 |               fn get_my_trait_1() -> impl MyTrait {
            //     |                                      ------------ the expected opaque type
            // ...
            // 890 |               fn get_my_trait_2() -> impl MyTrait {
            //     |                                      ------------ the found opaque type
            // ...
            // 896 | /                 if arg > 100 {
            // 897 | |                     get_my_trait_1()
            //     | |                     ---------------- expected because of this
            // 898 | |                 } else {
            // 899 | |                     get_my_trait_2()
            //     | |                     ^^^^^^^^^^^^^^^^ expected opaque type, found a different opaque type
            // 900 | |                 }
            //     | |_________________- `if` and `else` have incompatible types
            //     |
            //     = note: expected opaque type `impl MyTrait` (opaque type at <src/chapter_010.rs:885:36>)
            //             found opaque type `impl MyTrait` (opaque type at <src/chapter_010.rs:890:36>)
            //     = note: distinct uses of `impl Trait` result in different opaque types

            // For more information about this error, try `rustc --explain E0308`.
            // error: could not compile `learn-rust-book` due to previous error

            // Blanket impl
            let mo_3 = MyObject::new();
            let mo_3_string = mo_3.to_string();
            println!("mo_3_string: {}", mo_3_string);
        }
    }

}

fn chapter_010_3() {
    println!("10.3. Validating References with Lifetimes");

    fn largest<'a >(l: &'a str, r: &'a str) -> &'a str {
        if l.len() > r.len() {
            l
        } else {
            r
        }
    }

    let s1 = "Multiple words in a sentence".to_string();
    let s2 =  String::from("One");
    let s3 = largest(&s1, &s2);
    println!("s3: {}", s3);

    // Preventing Dangling references with Lifetimes
    {
        // {
        //     let r;
        //     {
        //         let x = 5;
        //         r = &x; // X: does not live long enough
        //     }
        //     println!("r: {}", r);
        // }
    }

    // Lifetime Annotation Syntax
    {
        // &i32;
        // &'a i32;
        // &'a mut i32;

        fn max_i32<'a >(first: &'a i32, second: &'a i32) -> &'a i32 {
            if first > second {
                first
            } else {
                second
            }
        }

        let i1 = 100;
        let i2 = 200;

        let r = max_i32(&i1, &i2);
        println!("r: {}", r);
    }

    // Thinking in Terms of Lifetime
    {
        fn longest_i32<'a>(first: &'a i32, _second: &i32) -> &'a i32 {
            first
        }

        let a = 100;
        let b = 200;
        let _c = longest_i32(&a, &b);
        // Lifetime 'a describes relationships between the first arg and the return value.

        // fn _tris_return_local_reference <'a> (x: &str) -> &'a str {
        //     let d = String::from("ASDF");
        //     d.as_str()
        // }
        // Error: Can't return references to data owned by the current function
    }

    // Lifetime Annotations in Struct Definitions
    #[allow(dead_code)]
    {
        struct Object <'a> {
            data: &'a str,
        }

        let str_data = String::from("A B C");
        let first_word = str_data.split(' ').next().unwrap();

        let _o1 = Object { data: "string slice" };
        let _o2 = Object { data: &str_data };
        let _o3 = Object { data: first_word };

        // _o3 can't live longer than the object that the inner data reference points to.
    }

    // Lifetime Elision
    {
        #[allow(clippy::redundant_slicing)]
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
            &s[..]
        }

        let s = "11 22 33";
        let r = first_word(s);
        println!("first_word of [{}] string is [{}]", s, r);
    }

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    {
        use std::fmt::Display;

        fn largest <'a, T> (
            x: &'a str, 
            y: &'a str, 
            ann: T
        ) -> &'a str 
            where T: Display
        {
            println!("ann: {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let s1 = "11111";
        let s2 = "15";
        let _r = largest(s1, s2, "Rust Lifetime Annotations");
        println!("{}", _r);
    }
}
