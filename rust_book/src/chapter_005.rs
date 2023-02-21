pub fn run () {
    println!("5. Using Structs to Structure Related Data");
    chapter_5_1();
    chapter_5_2();
    chapter_5_3();
}

fn chapter_5_1 () {
    println!("5.1. Defining and Instantiating Structs");
    let user1 = User {
        active: false,
        username: String::from("User 1"),
        email: String::from("user1@a.com"),
        sign_in_count: 0,
    };

    let mut user2 = build_user(String::from("User 2"), String::from("user2@b.com"));
    println!("user1: {:?}", (&user1.active, &user1.username, &user1.email, &user1.sign_in_count));
    println!("user2: {:?}", (&user2.active, &user2.username, &user2.email, &user2.sign_in_count));
    user2.email = String::from("user2-modified@email.com");
    println!("user2 (modified): {:?}", (&user2.active, &user2.username, &user2.email, &user2.sign_in_count));
    
    // Struct update syntax
    let user3 = User {
        username: String::from("User 3"),
        ..user2
    };
    println!("user3 (syntax update): {:?}", (&user3.active, &user3.username, &user3.email, &user3.sign_in_count));
    // println!("user2 (modified): {:?}", (&user2.active, &user2.username, &user2.email, &user2.sign_in_count)); // User2 invelidated after ..user2

    // Tuple Structs
    struct Color (u8, u8, u8);
    let green: Color = Color (0, 255, 0);
    println!("green: {} {} {}", green.0, green.1, green.2);
    
    // Destruct Tuple Struct
    let Color(r, g, b) = green; // (green.0, green.1, green.2);
    println!("r: {}, g: {}, b: {}", r, g, b);

    // Unit-like Structs
    #[derive(Debug)]
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
    println!("_subject: {:?}", _subject);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    return User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// ----------------------------------------------------------------

fn chapter_5_2() {
    println!("5.2. An Example Program Using Structs");
    rectangles();
}

fn rectangles() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32, height: u32,
    }
    #[derive(Debug)]
    struct Object {
        id: usize,
        name: String,
        rect: Rectangle,
    }
    fn area_wh(w: u32, h: u32) -> u32 { w * h }
    fn area_dim(dim: (u32, u32)) -> u32 { dim.0 * dim.1 }
    fn area(rect: &Rectangle) -> u32 { rect.width * rect.height }

    let w1 = 30;
    let h1 = 50;
    println!("Rect area, WH: {}", area_wh(w1, h1));
    
    let rect1 = (30, 50);
    println!("Rect area, Dimensions: {}", area_dim(rect1));
    
    let rect2 = Rectangle { width: 30, height: 50 };
    println!("Rect area, Struct: {}", area(&rect2));

    println!("rect2, debug-print: {:?}", rect2);
    println!("rect2, debug-pretty-print: {:#?}", rect2);

    let object = Object {
        id: 100,
        name: String::from("Object Name"),
        rect: Rectangle {
            width: 100, height: 200,
        },
    };
    println!("object: {:?}", object);
    println!("object fields: {}, {}, {:?}", object.id, object.name, object.rect);
    println!("pretty-print: object: {:#?}", object);

    dbg!(&object);

    let add_result = dbg!(100 + 256);
    println!("add_result: {:?}", add_result);

    let s1 = String::from("String s1");
    let s2 = dbg!(s1);
    // println!("s1: {:?}", s1); Error: borrow of move value s1, borrow happened here: dbg!(s1)
    println!("s2: {:?}", s2);
}

// ----------------------------------------------------------------

fn chapter_5_3() {
    println!("5.3. Method Syntax");

    #[derive(Debug)]
    struct Rectangle {
        width: i32,
        height: i32,
    }

    impl Rectangle {
        fn area(&self) -> i32 {
            self.width * self.height
        }

        // Overide default getter behavoiur for the investigational purposes only
        fn width(&self) -> bool {
            self.width > 0
        }

        fn height(self: &Self) -> i32 {
            self.height
        }

        fn can_hold(&self, r: &Rectangle) -> bool {
            self.width > r.width && self.height > r.height
        }

        fn info(&self) -> String {
            format!("{:p}: Rectangle {{ width: {}, height: {} }}", &self, self.width, self.height)
        }
    }
    
    // Adding another impl block for the Rectangle struct
    impl Rectangle {
        fn square(size: i32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    let rect = Rectangle {
        width: 500,
        height: 350,
    };
    
    println!("rect: {:#?}", rect); // Pretty debug output
    println!("rect.area(): {}", rect.area());
    println!("rect.width: {}", rect.width);
    println!("rect.width(): {}", rect.width());
    println!("rect.height: {}", rect.height);
    println!("rect.height(): {}", rect.height());
    println!("rect.height(): {}", rect.height());

    // Can hold test...
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated function...
    let square_rect = Rectangle::square(500);
    println!("square_rect: {:?}", square_rect);
    println!("square_rect: {:?}", square_rect.info());
}
