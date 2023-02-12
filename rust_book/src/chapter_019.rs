use std::{slice, ops::{Deref, DerefMut}, rc::Rc};

pub fn run() {
    println!("19. Advanced Features");
    chapter_019_1();
    chapter_019_2();
    chapter_019_3();
    chapter_019_4();
}

fn chapter_019_1() {
    println!("19.1 - Unsafe Rust");

    // Raw Pointers
    {
        let mut i = 5;

        // Create a raw pointer
        let raw_const = &i as *const i32;
        let raw_mut = &mut i as *mut i32;
        let raw_mut_2 = raw_mut;

        // Ok
        println!("raw_const = {:p}", raw_const);
        println!("raw_mut = {:p}", raw_mut);

        // Error:
        // this operation is unsafe and requires an unsafe function or block
        // let i2 = *raw_const;

        // Some invalid addr
        let address_value = 0x1000usize;
        let address_pointer = address_value as *const i32;
        println!("address_value = {}", address_value);
        println!("address_pointer = {:p}", address_pointer);
        // Error: Segmentation fault (core dumped)
        // unsafe { println!("*address_pointer = {}", *address_pointer); }

        // Print data referenced by raw pointer
        unsafe {
            println!("*raw_const = {}", *raw_const);
            println!("*raw_mut = {}", *raw_mut);
            println!("*raw_mut_2 = {}", *raw_mut_2);
            *raw_mut += 1;
            println!("*raw_const = {}", *raw_const);
            println!("*raw_mut = {}", *raw_mut);
            println!("*raw_mut_2 = {}", *raw_mut_2);
            *raw_mut_2 += 1;
            println!("*raw_const = {}", *raw_const);
            println!("*raw_mut = {}", *raw_mut);
            println!("*raw_mut_2 = {}", *raw_mut_2);
        }
    }

    // Calling an Unsafe Function or Method
    {
        // Define unsafe function
        unsafe fn unsafe_foo() {
            println!("unsafe fn unsafe_foo called");
        }

        // Call within an unsafe block
        unsafe {
            unsafe_foo();
        }
    }

    // Creating a Safe Abstraction over Unsafe Code
    {
        // Safe code:
        // Error: cannot borrow `*values` as mutable more than once at a time
        // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        //     let len = values.len();
        //     assert!(mid <= len);
        //     (&mut values[..mid], &mut values[mid..])
        // }

        // Unsafe code
        let mut values = [0, 1, 2, 3, 4, 5, 6, 7];

        fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = values.len();
            let ptr = values.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                let left_slice = slice::from_raw_parts_mut(ptr, mid);
                let p2 = ptr.add(mid);
                let right_len = len - mid;
                let right_slice = slice::from_raw_parts_mut(p2, right_len);
                (left_slice, right_slice)
            }
        }

        let vl = values.len();
        for i in 0..vl {
            let (left, right) = split_at_mut(&mut values, i);
            println!("i = {}, left = {:?}, right = {:?}", i, left, right);
        }
    }

    // Unsafe code example
    {
        // Some invalid addr
        let address_value = 0x1000usize;
        let address_pointer = address_value as *const i32;
        let address_pointer_mut = address_value as *mut i32;
        println!("address_value = {}", address_value);
        println!("address_pointer = {:p}", address_pointer);
        let s1000 = unsafe { slice::from_raw_parts_mut(address_pointer_mut, 1000) };
        println!("s1000.len = {:?}", s1000.len());
        // Binary has crashed:
        // println!("s1000[0] = {:?}", s1000[0]);
    }

    // Using extern Functions to Call External Code
    {
        extern "C" {
            fn abs(i: i32) -> i32;
        }

        let i = unsafe { abs(10) };
        println!("i = {}", i);

        let i = unsafe { abs(-11) };
        println!("i = {}", i);
    }

    // Accessing or Modifying a Mutable Static Variable
    {
        static mut COUNTER: i32 = 0;
        const CONSTANT_VALUE: i32 = 10;

        println!("COUNTER = {:p}", unsafe { &COUNTER });
        println!("CONSTANT_VALUE = {:p}", &CONSTANT_VALUE);
        
        unsafe {
            COUNTER += 1;
        }

        println!("COUNTER = {:p}", unsafe { &COUNTER });
        println!("CONSTANT_VALUE = {:p}", &CONSTANT_VALUE);
    }

    // Implementing an Unsafe Trait
    {
        // Send + Sync
        struct A {}
        unsafe impl Send for A {}
        unsafe impl Sync for A {}

        // Unsafe trait
        unsafe trait ThisIsUnsafe {}
        unsafe impl ThisIsUnsafe for A {}
    }

    // Accessing Fields of a Union
    {
        use std::fmt;

        union A {
            i_32: i32,
            i_16: i16,
            i_8: i8,
            u_8: u8,
            i_t_4: (i8, i8, i8, i8),
            u_t_4: (u8, u8, u8, u8),
            i_a_4: [i8; 4],
        }

        impl std::fmt::Display for A {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                unsafe {
                    write!(f, "union A {{\n")?;
                    write!(f, "    i_32  = {:?}\n", self.i_32)?;
                    write!(f, "    i_16  = {:?}\n", self.i_16)?;
                    write!(f, "    i_8   = {:?}\n", self.i_8)?;
                    write!(f, "    u_8   = {:?}\n", self.u_8)?;
                    write!(f, "    i_t_4 = {:?}\n", self.i_t_4)?;
                    write!(f, "    u_t_4 = {:?}\n", self.u_t_4)?;
                    write!(f, "    i_a_4 = {:?}\n", self.i_a_4)?;
                    write!(f, "}} union A\n")
                }
            }
            // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            //     // Use `self.number` to refer to each positional data point.
            //     write!(f, "({}, {})", self.0, self.1)
            // }
        }

        let mut a = A {
            i_t_4: (1, 1, 1, 1)
        };

        let a_i32 =  unsafe {
            a.i_32
        };

        println!("1. a_i32: {:?}", a_i32);
        println!("1. a: {}", a);

        a.i_32 = 0;
        a.i_32 = 268435456;
        println!("2. a_i32: {:?}", unsafe { a.i_32 });
        println!("2. a: {}", a);

        a.i_32 = 0;
        a.i_16 = 257;
        println!("3. a_i16: {:?}", unsafe { a.i_16 });
        println!("3. a: {}", a);

        for i in 0..257 {
            let u = A { i_32: i };
            println!("u: {}", u);
        }
    }
}

fn chapter_019_2() {
    println!("19.2. Advanced Traits");

    // Specifying Placeholder Types in Trait Definitions with Associated Types
    #[allow(dead_code)]
    {
        #[derive(Debug)]
        struct Counter {
            i: i32,
            limit: i32,
        }

        impl Counter {
            pub fn new() -> Self {
                Self { 
                    i: 0,
                    limit: 1024,
                }
            }

            pub fn limit(&self) -> i32 {
                self.limit
            }

            pub fn increment(&mut self) -> i32 {
                self.i += 1;
                self.i
            }

            pub fn decrement(&mut self) -> i32 {
                self.i -= 1;
                self.i
            }
        }

        pub trait Iterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item>;
        }

        impl Iterator for Counter {
            type Item = i32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.i < self.limit {
                    Some(self.increment())
                } else {
                    None
                }
            }
        }

        let mut counter = Counter::new();
        counter.increment();

        while let Some(i) = counter.next() {
            println!("i: {}", i);
        }
    }

    // Default Generic Type Parameters and Operator Overloading
    {
        use crate::common::chapter_019::Point;

        // Explicit impls
        {
            let p_i32 = Point::<i32, i32>::new(1, 2);
            let p_f64 = Point::<f64, f64>::new(10.5, 2.5);
            let p_other = Point::new(100usize, 200isize);
    
            p_i32.works_when_i32();
            p_i32.works_when_t_and_u();
    
            p_f64.works_when_f64();
            p_f64.works_when_t_and_u();
    
            p_other.works_when_t_and_u();
        }

        // Add operator overloading
        {
            use std::ops::Add;

            impl Add for Point {
                type Output = Point<i32, i32>;
                fn add(self, other: Point<i32, i32>) -> Self::Output {
                    Point::new(
                        self.x + other.x, 
                        self.y + other.y
                    )
                }
            }

            impl std::ops::Sub for Point {
                type Output = Point;
                fn sub(self, other: Point) -> Self::Output {
                    Point::new(self.x - other.x, self.y - other.y)
                }
            }

            let p1 = Point::new(1, 2);
            let p2 = Point::new(3, 4);
            
            let p3 = p1 + p2;
            assert_eq!(p3, Point::new(4, 6));

            let p4 = p1 - p2;
            assert_eq!(p4, Point::new(-2, -2));

            println!("p1: {:?}", p1);
            println!("p2: {:?}", p2);
            println!("p3: {:?}", p3);
            println!("p4: {:?}", p4);
        }

        // Customize Rhs
        {
            #[derive(Clone, Copy)]
            struct Millimeter(f64);

            #[derive(Clone, Copy)]
            struct Meters(f64);

            impl std::fmt::Display for Millimeter {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_fmt(
                        format_args!("{} mm", self.0)
                    )
                }
            }

            impl std::fmt::Display for Meters {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_fmt(
                        format_args!("{} m", self.0)
                    )
                }
            }

            impl std::ops::Add for Millimeter {
                type Output = Millimeter;
                fn add(self, other: Millimeter) -> Self::Output {
                    Millimeter(self.0 + other.0)
                }
            }

            impl std::ops::Sub for Millimeter {
                type Output = Millimeter;
                fn sub(self, other: Millimeter) -> Self::Output {
                    Millimeter(self.0 - other.0)
                }
            }

            impl std::ops::Add for Meters {
                type Output = Meters;
                fn add(self, other: Meters) -> Self::Output {
                    Meters(self.0 + other.0)
                }
            }

            impl std::ops::Sub for Meters {
                type Output = Meters;
                fn sub(self, other: Meters) -> Self::Output {
                    Meters(self.0 - other.0)
                }
            }

            // Rhs is Meters
            impl std::ops::Add<Meters> for Millimeter {
                type Output = Millimeter;
                // Rhs is Meters
                fn add(self, other: Meters) -> Self::Output {
                    Millimeter(self.0 + other.0 * 1000.0)
                }
            }

            // Rhs is Meters
            impl std::ops::Sub<Meters> for Millimeter {
                type Output = Millimeter;
                // Rhs is Meters
                fn sub(self, other: Meters) -> Self::Output {
                    Millimeter(self.0 - other.0 * 1000.0)
                }
            }

            // Rhs is Millimeter
            impl std::ops::Add<Millimeter> for Meters {
                type Output = Millimeter;
                // Rhs is Millimeter
                fn add(self, other: Millimeter) -> Self::Output {
                    Millimeter(self.0 + other.0 / 1000.0)
                }
            }

            // Rhs is Millimeter
            impl std::ops::Sub<Millimeter> for Meters {
                type Output = Millimeter;
                // Rhs is Millimeter
                fn sub(self, other: Millimeter) -> Self::Output {
                    Millimeter(self.0 - other.0 / 1000.0)
                }
            }

            let mm1 = Millimeter(115.0);
            let mm2 = Millimeter(125.0);
            let m1 = Meters(1.35);
            let m2 = Meters(1.25);

            println!("mm1 + mm2 = {}", mm1 + mm2);
            println!("{} + {} = {}", mm1, mm2, mm1 + mm2);

            println!("mm1 - mm2 = {}", mm1 - mm2);
            println!("{} - {} = {}", mm1, mm2, mm1 - mm2);

            println!("m1 + m2 = {}", m1 + m2);
            println!("{} + {} = {}", m1, m2, m1 + m2);

            println!("m1 - m2 = {}", m1 - m2);
            println!("{} - {} = {}", m1, m2, m1 - m2);

            println!("mm1 + m1 = {}", mm1 + m1);
            println!("{} + {} = {}", mm1, m1, mm1 + m1);

            println!("mm1 - m1 = {}", mm1 - m1);
            println!("{} - {} = {}", mm1, m1, mm1 - m1);

            println!("m2 + mm2 = {}", m2 + mm2);
            println!("{} + {} = {}", m2, mm2, m2 + mm2);

            println!("m2 - mm2 = {}", m2 - mm2);
            println!("{} - {} = {}", m2, mm2, m2 - mm2);
        }
    }

    // Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
    {
        trait A {
            fn print_me(&self);
            fn associated();
        }
        
        trait B {
            fn print_me(&self);
            fn associated();
        }

        struct C {
        }

        impl A for C {
            fn print_me(&self) {
                println!("impl A for C :: print_me");
            }
            fn associated() {
                println!("impl A for C :: associated");
            }
        }

        impl B for C {
            fn print_me(&self) {
                println!("impl B for C :: print_me");
            }
            fn associated() {
                println!("impl B for C :: associated");
            }
        }

        impl C {
            fn print_me(&self) {
                println!("impl C :: print_me");
            }
            fn associated() {
                println!("impl C :: associated");
            }
        }

        let c = C {};

        // Methods
        c.print_me();
        A::print_me(&c);
        B::print_me(&c);
        C::print_me(&c);

        // Associated functions
        <C as A>::associated();
        <C as B>::associated();
        C::associated();
    }

    // Using Supertraits to Require One Trait’s Functionality Within Another Trait
    {
        // Supertrait - Outlined example
        {
            use crate::common::chapter_019::Point;
    
            trait OutlinedPrint: std::fmt::Display {
                fn outline_print(&self) {
                    let output = self.to_string();
                    let len = output.len();
                    println!("{}", "*".repeat(len + 4));
                    println!("*{}*", " ".repeat(len + 2));
                    println!("* {} *", output);
                    println!("*{}*", " ".repeat(len + 2));
                    println!("{}", "*".repeat(len + 4));
                }
            }
    
            impl std::fmt::Display for Point {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "(x = {}, y = {})", self.x, self.y)
                }
            }
    
            impl OutlinedPrint for Point {}
    
            let p = Point { x: 1, y: 2 };
            println!("p = {}", p);
            p.outline_print();
        }

        //
        {
            trait SomeSuperTrait {
                fn this_is_super_required(&self);
            }

            trait MyTrait: SomeSuperTrait {
                fn this_is_required(&self) {
                    self.this_is_super_required();
                }
            }

            struct A;

            impl SomeSuperTrait for A {
                fn this_is_super_required(&self) {
                    println!("impl SomeSuperTrait for A :: this_is_super_required");
                }
            }

            impl MyTrait for A {}

            let a = A {};
            a.this_is_required();
        }
    }

    // Using the Newtype Pattern to Implement External Traits on External Types
    {
        // 1. Define wrapper for an external type.
        #[derive(Debug)]
        struct VecOfi32(Vec<i32>);

        impl VecOfi32 {
            pub fn with_capacity(capacity: usize) -> Self {
                VecOfi32(Vec::with_capacity(capacity))
            }

            pub fn capacity(&self) -> usize {
                self.0.capacity()
            }
        }

        // 2. Implement external trait on a wrapper.
        impl std::fmt::Display for VecOfi32 {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let vec_of_i32_as_strings = self.0.iter().map(|i| i.to_string()).collect::<Vec<String>>();
                write!(f, "*** >>> {:?} <<< ***", vec_of_i32_as_strings.join(" then "))
            }
        }

        // 3. Implement deref to be able to access inner value
        impl Deref for VecOfi32 {
            type Target = Vec<i32>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for VecOfi32 {
            // type Target = Vec<i32>;
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        let v_of_i32 = VecOfi32(vec![10, 100, 1000, 10000]);
        println!("v_of_i32: {}", &v_of_i32);
        dbg!("DEBUG: v_of_i32: {:?}", &v_of_i32);

        let ref v_of_32_inner = *v_of_i32;
        println!("v_of_32_inner: {:?}", v_of_32_inner);
        println!("v_of_32_inner.len = {}", v_of_32_inner.len());

        let mut v_of_i32 = VecOfi32::with_capacity(10);
        println!("1. v_of_i32.capacity: {}, items: {:?}", v_of_i32.capacity(), v_of_i32);
        v_of_i32.push(10);
        v_of_i32.push(20);
        println!("2. v_of_i32.capacity: {}, items: {:?}", v_of_i32.capacity(), v_of_i32);
    }
}

fn chapter_019_3() {
    // Creating Type Synonyms with Type Aliases
    {
        // 1
        {
            type Km = i32;
            let x = 10;
            let y: Km = x;
            let z = 10;
            println!("x, y, z: {:?}", (x, y, z));
            println!("x + y + z: {:?}", x + y + z);
        }

        // 2
        {
            type CloType = Box<dyn Fn() + Send + 'static>;
            let f: CloType = Box::new(|| println!("test"));

            fn takes_f_returns_f<'a>(
                f: &'a Box<dyn Fn() + Send + 'static>
            ) -> &'a Box<dyn Fn() + Send + 'static>
            {
                f();
                f
            }

            fn takes_f_returns_f_too<'a>(f: &'a CloType) -> &'a CloType {
                f();
                f
            }

            takes_f_returns_f(&f)();
            takes_f_returns_f_too(&f)();

            // Prints test four times.
        }

        // 3 - reduce repetitions
        {
            // Definition contains long type names.
            pub trait Write {
                fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error>;
                fn flush(&mut self) -> std::result::Result<(), std::io::Error>;
            }

            type MyResult<T> = std::result::Result<T, std::io::Error>;

            // Definition contains short type names.
            pub trait WriteToo {
                fn write(&mut self, buf: &[u8]) -> MyResult<usize>;
                fn flush(&mut self) -> MyResult<()>;
            }

            // Usage
            #[derive(Default)]
            pub struct MyWrite {}

            impl WriteToo for MyWrite {
                fn write(&mut self, buf: &[u8]) -> MyResult<usize> {
                    Ok(buf.len())
                }
                fn flush(&mut self) -> MyResult<()> {
                    Ok(())
                }
            }

            let mut mw = MyWrite::default();
            match mw.write(&[1u8, 2u8, 3u8]) {
                Ok(n) => println!("written {} bytes", n),
                Err(_) => println!("error"),
            }
        }
    }

    // The Never Type that Never Returns
    {
        // 1
        {
            fn _foo() -> ! {
                let mut i = 0;
                loop {
                    i += 1;
                    println!("called {:?} times", i);
                }
            }
            // Enters infinite loop
            // _foo();
        }

        // 2
        {
            let guess = String::from("1024");
            loop {
                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(e) => {
                        println!("error: {:?}", e);
                        continue
                    //  ^^^^^^^^ Returns !-type
                    },
                };
                println!("guess: {}", guess);
                break;
            }
        }
    }

    // Dynamically Sized Types and the Sized Trait
    {
        // 1 - Sized Types
        {
            fn foo<T> (arg: T) -> T
            where
                T: std::fmt::Debug,
            {
                println!("arg: {:?}", arg);
                arg
            }
            
            fn foo2<T>(arg: T) -> T
            where
            T: std::fmt::Debug + Sized,
            {
                println!("arg: {:?}", arg);
                arg
            }

            foo("arg1".to_string());
            foo2("arg2".to_string());
            assert_eq!(foo("arg3".to_string()), foo2("arg3".to_string()));
        }

        // 2 - Unsized examples - ?Sized
        {
            fn usized_with_ref_t<'a, T: ?Sized + std::fmt::Debug> (arg: &'a T) -> &'a T {
                println!("usized_with_ref_t :: arg: {:?}", &arg);
                arg
            }

            fn usized_with_box_t<'a, T:?Sized + std::fmt::Debug> (arg: Box<T>) -> Box<T> {
                println!("usized_with_box_t :: arg: {:?}", &arg);
                arg
            }

            fn usized_with_rc_t<'a, T:?Sized + std::fmt::Debug> (arg: Rc<T>) -> Rc<T> {
                println!("usized_with_rc_t :: arg: {:?}", &arg);
                arg
            }

            let s1 = "arg1";
            let s2 = Box::new("arg22");
            let s3 = Rc::new("arg33");

            let r1 = usized_with_ref_t(s1);
            let r2 = usized_with_box_t(s2);
            let r3 = usized_with_rc_t(s3);

            println!("r1: {:?}", r1);
            println!("r2: {:?}", r2);
            println!("r3: {:?}", r3);
        }
    }
}

/// 19.4. Advanced Functions and Closures
fn chapter_019_4() {
    // Function Pointers
    {
        
        // 1
        {
            fn add_one(x: i32) -> i32 { 
                x + 1
            }
    
            fn add_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
                f(arg) + f(arg)
            }
    
            let mult_by_2: fn(i32) -> i32 = |x: i32| -> i32 {
                x * 2
            };
    
            let r1 = add_twice(add_one, 100);
            let r2 = add_twice(mult_by_2, 555);
    
            println!("r1: {:?}", r1);
            println!("r2: {:?}", r2);
        }

        // 2
        {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8,];
            let vs1 = v.iter().map(|x| x.to_string()).collect::<Vec<String>>();

            fn my_to_string(x: &i32) -> String {
                format!(" :: {} :: ", x.to_string())
            }
            let vs2 = v.iter().map(my_to_string).collect::<Vec<String>>();

            println!("vs1: {:?}", vs1);
            println!("vs2: {:?}", vs2);
        }

        // 3
        {
            #[derive(Debug)]
            enum States {
                _Start,
                Progress(isize),
                Result(String),
            }

            // let s1 = States::Start;
            // let s2 = States::Progress(25);
            // let s3 = States::Result("Hello".to_string());

            let v_s_progress = (97..=100)
                .map(States::Progress)
                .collect::<Vec<States>>();
                
            let v_s_res = &["str1".to_owned(), "str2".to_owned()]
                .map(States::Result);

            println!("v_s_progress: {:?}", v_s_progress);
            println!("v_s_res: {:?}", v_s_res);
        }
    }

    // Returning closures
    {
        // Error
        // fn returns_closure() -> dyn Fn(i32) -> i32 {
        //     |x| x + 1
        // }

        // 1 Returns closure behind the impl
        {
            fn returns_closure() -> impl Fn(i32) -> i32 {
                |x| x + 1
            }
            
            let c = returns_closure();
            let i = c(100);
            
            println!("i: {:?}", i);
        }

        // 2 Returns Boxed closure
        {
            fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
                Box::new(|x| x + 1)
            }

            let c = returns_closure();
            let i = c(10);
            
            println!("i: {:?}", i);
        }
    }
}
