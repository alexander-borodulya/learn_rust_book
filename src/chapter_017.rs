pub fn run() {
    println!("Chapter 17 - Fearless Concurrency");
    chapter_017_1();
    chapter_017_2();
    chapter_017_3();
}

#[allow(dead_code)]
fn chapter_017_1() {
    println!("Chapter 17.1 - todo...");

    pub struct AvaragedCollection {
        list: Vec<i32>,
        avarage: f64,
    }

    impl AvaragedCollection {
        pub fn new() -> AvaragedCollection {
            AvaragedCollection {
                list: Vec::new(),
                avarage: 0.0,
            }
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_avarage();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_avarage();
                    Some(value)
                },
                None => None,
            }
        }

        pub fn avarage(&self) -> f64 {
            self.avarage
        }

        fn update_avarage(&mut self) {
            let sum = self.list.iter().sum::<i32>();
            self.avarage = sum as f64 / self.list.len() as f64
        }
    }

    let mut ac = AvaragedCollection::new();
    println!("[0] ac.avarage: {}", ac.avarage());
    ac.add(1);
    println!("[1] ac.avarage: {}", ac.avarage());
    ac.add(2);
    println!("[2] ac.avarage: {}", ac.avarage());
    ac.add(500);
    println!("[3] ac.avarage: {}", ac.avarage());
    ac.add(501);
    println!("[4] ac.avarage: {}", ac.avarage());
    let removed = ac.remove();
    println!("[5] removed: {:?}", removed);
    println!("[5] ac.avarage: {}", ac.avarage());
    
    while let Some(removed) = ac.remove() {
        println!(" - [6] removed: {:?}", removed);
        println!(" - [6] ac.avarage: {}", ac.avarage());
    }
    println!("[7] ac.avarage: {}", ac.avarage());
    
}

#[allow(dead_code)]
fn chapter_017_2() {
    println!("Chapter 17.2 - todo...");

    {
        use learn_rust_book::rust_book::chapter_017::ch_017_02::{Screen, Button, Draw};

        let mut screen = Screen::new(
            vec![
                Box::new(Button::new()),
                Box::new(Button::new_with_args(128, 24, "Wide Button".to_owned())),
            ]
        );

        println!("[1]");

        screen.run();

        println!("[2]");
        
        #[derive(Debug)]
        struct SelectBox {
            width: usize,
            height: usize,
            options: Vec<String>,
        }

        impl SelectBox {
            fn new() -> Self {
                Self {
                    width: 32,
                    height: 24,
                    options: vec!["Yes".to_owned(), "No".to_owned(), "Maybe".to_owned()],
                }
            }
        }

        impl Draw for SelectBox {
            fn draw(&self) {
                println!("SelectBox: draw: {:?}", (self.width, self.height, &self.options));
            }
        }
        
        screen.components.push(Box::new(SelectBox::new()));
        screen.run();

        println!("[3]");
    }
    
    // Vector of &-references
    {
        use learn_rust_book::rust_book::chapter_017::ch_017_02::{Button, Draw, Label};

        let mut components: Vec<&dyn Draw> = vec![];

        let b1 = Button::new();
        let b2 = Button::new_with_args(200, 100, String::from("Button 2"));
        let l1 = Label::new();
        let s1 = String::from("Just String");

        components.push(&b1);
        components.push(&b2);
        components.push(&l1);
        components.push(&s1);

        println!("[2.1]");
        println!("components: {:?}", components);
        println!("[2.2]");
        for c in components.into_iter() {
            c.draw();
        }
        println!("[2.3] - done");
    }
    
}

fn chapter_017_3() {
    println!("Chapter 17.3 - todo...");

    // Box holding a type
    {
        /////////////////////////////////////////////////////////////////////
        // 
        // Struct case
        //
        #[derive(Debug)]
        struct A {
            data: String,
        }

        impl A {
            fn new(data: String) -> Self {
                Self { data }
            }

            fn consume(self) {
                println!("A: consume: {:?}", self.data);
            }

            fn consume_boxed(self: Box<Self>) {
                println!("A: consume_boxed: {:?}", self.data);
            }

            fn println_data(&self) {
                println!("A: data: {:?}", self.data);
            }
        }

        let a = A::new("Hello A".to_string());
        a.println_data();
        a.consume();
        // a.println_data(); // Error: borrow of moved value
        
        //
        // Code below
        //
        // let a = A::new("Hello A".to_string());
        // a.consume_boxed();
        // 
        // Generates the following output:
        //
        //     no method named `consume_boxed` found for struct `A` in the current scoperustcClick for full compiler diagnostic
        //     chapter_017.rs(155, 9): method `consume_boxed` not found for this struct
        //     chapter_017.rs(168, 16): the method is available for `Box<A>` here
        //     chapter_017.rs(175, 9): consider wrapping the receiver expression with the appropriate type: `Box::new(`, `)`
        //     chapter_017.rs(175, 11): there is a method with a similar name: `consume`
        //

        let mut a = Box::new(A::new("Hello Boxed A".to_string()));
        {
            let a = a.as_mut();
            a.data.push_str(", add via as_ref");
        }

        a.println_data();
        a.consume();
        // a.println_data();
        // Error:
        //     borrow of moved value: `*a`
        //     move occurs because `*a` has type `A`, which does not implement the `Copy` traitrustcClick for full compiler diagnostic
        //     chapter_017.rs(197, 11): `*a` moved due to this method call
        //     chapter_017.rs(164, 24): this function takes ownership of the receiver `self`, which moves `*a`
        
        let a = Box::new(A::new("Hello Boxed A".to_string()));
        a.println_data();
        a.consume_boxed();
        // a.println_data();
        // Error:
        //     borrow of moved value: `*a` ... (the same as above)

        /////////////////////////////////////////////////////////////////////
        //
        // Function case
        //

        fn test<T>(arg: Box<T>)
        where
            T: std::fmt::Debug,
        {
            println!("T: {:?}", arg);
        }

        let b1 = Box::new(100);
        let b2= Box::new(String::from("Hello"));
        let b3 = Box::new(A::new("some string".to_owned()));

        test(b1);
        test(b2);
        test(b3);
    }

    // Demonstrate state changes: v1 and v3 implementations
    {
        // use learn_rust_book::rust_book::chapter_017::ch_017_03::v1::Post;
        // use learn_rust_book::rust_book::chapter_017::ch_017_03::v3::Post;
        use learn_rust_book::rust_book::chapter_017::ch_017_03::v4::Post;

        let mut post = Post::new();
        println!("post.state: {:?}", post.state());
        
        post.add_text("I read Rust Book everyday!");
        println!("post.state: {:?}", post.state());
        println!("post.content: {:?}", post.content());
        assert_eq!(post.content(), "");
        
        post.request_review();
        println!("post.state: {:?}", post.state());
        println!("post.content: {:?}", post.content());
        assert_eq!(post.content(), "");
        post.add_text("The add_text function is called, but ignored becase it is not a Draft state.");
        
        post.reject();
        println!("post.state: {:?}", post.state());
        println!("post.content: {:?}", post.content());
        assert_eq!(post.content(), "");
        
        post.add_text(" After that, I program in Rust!");
        
        post.approve();
        println!("post.state (after reject): {:?}", post.state());
        println!("post.content (after reject): {:?}", post.content());
        assert_eq!(post.content(), "");
        
        post.request_review();
        println!("post.state (after request_review): {:?}", post.state());
        println!("post.content (after request_review): {:?}", post.content());
        assert_eq!("", post.content());
        post.add_text("The add_text function is called, but ignored becase it is not a Draft state.");
        
        post.approve();
        println!("post.state (after approve, 1st): {:?}", post.state());
        println!("post.content (after approve, 1st): {:?}", post.content());
        assert_eq!(post.content(), "");
        post.add_text("The add_text function is called, but ignored becase it is not a Draft state.");
        
        post.approve();
        println!("post.state (after approve, 2nd): {:?}", post.state());
        println!("post.content (after approve, 2nd): {:?}", post.content());
        assert_eq!(post.content(), "I read Rust Book everyday! After that, I program in Rust!");
        post.add_text("The add_text function is called, but ignored becase it is not a Draft state.");
    }
    
    // Encoding States and Behavior as Types
    {
        use learn_rust_book::rust_book::chapter_017::ch_017_03::v2::Post;
        
        let mut post = Post::new();
        
        post.add_text("I read Rust Book everyday!");

        let post = post.request_review();

        let mut post = post.reject();

        post.add_text(" After that, I program in Rust!");

        let post = post.request_review();

        let post = post.approve();

        let post = post.approve();

        println!("post.content: {:?}", post.content());
        assert_eq!(post.content(), "I read Rust Book everyday! After that, I program in Rust!");
    }
}
