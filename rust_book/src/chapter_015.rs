use std::{ops::{Add, AddAssign}, rc::{Rc, Weak}, cell::RefCell};

pub fn run() {
    println!("15. Smart Pointers");
    chapter_015_1();
    chapter_015_2();
    chapter_015_3();
    chapter_015_4();
    chapter_015_5();
    chapter_015_6();
    debuging_struct_that_holds_reference();
}

fn chapter_015_1() {
    println!("15.1. Using Box<T> to Point to Data on the Heap");
    
    // Using box<T> to Store Data on the Heap
    {
        let b_i32 = Box::new(128);
        let b_usize = Box::new(256_usize);
        println!("b_i32: {}", b_i32);
        println!("b_usize: {}", b_usize);

        #[allow(dead_code)]
        #[derive(Debug)]
        struct Object {
            id: usize,
        }
        let b_obj = Box::new( Object { id: 100 } );
        println!("b_obj: {:?}", b_obj);
    }

    // Enabling Recursive Types with Boxes
    {
        mod recursive_types_ch15_1 {

            #[derive(Debug)]
            pub enum List {
                Cons(i32, Box<List>),
                Nil
            }
        }

        use recursive_types_ch15_1::List::{Cons, Nil};

        let cons_list = Box::new(Cons(100, Box::new(Cons(200, Box::new(Cons(300, Box::new(Nil)))))));
        println!("cons_list: {:?}", cons_list);
    }

    // Computing the Size of Non-Recursive Type
    {
        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
    }

    let mut b_i32 = Box::new(128);
    println!("b_i32: {:?}", b_i32);
    b_i32.add_assign(256);
    println!("b_i32: {:?}", b_i32);
    let sum = b_i32.add(512);
    println!("b_i32: {:?}, sum: {:?}", b_i32, sum);

}

/// Runs code for the chapter_015_2
fn chapter_015_2() {
    println!("15.2. Treating Smart Pointers Like Regular References with the Deref Trait");
    
    // Following the Pointer of the Value
    {
        let x = 5;
        println!("x: {:?}", x);
        println!("&x: {:?}", &x);
        
        let x_r = &x;
        println!("x_r: {:?}", x_r);
        println!("*x_r: {:?}", *x_r);
    
        let x_r2 = &x_r;
        let x_r1 = *x_r2;
        println!("x_r2: {:?}, x_r1: {:?}", x_r1, x_r2);

        let x_r3 = &x_r2;
        println!("x_r3: {:?}", x_r3);
        println!("*x_r3: {:?}", *x_r3);
        println!("**x_r3: {:?}", **x_r3);
        println!("***x_r3: {:?}", ***x_r3);
    }
    
    // Using Box<T> Like a Reference
    {
        // 1
        let x = 5;
        println!("x: {:?}", x);
        println!("&x: {:?}", &x);
        
        // 2
        let box_y = Box::new(x);
        println!("box_y: {:?}", box_y);
        println!("*box_y: {:?}", *box_y);
        println!("&box_y: {:?}", &box_y);
        
        // 3
        let box_ref_x = Box::new(&x);
        println!("x: {:?}", x);
        println!("box_ref_x: {:?}", box_ref_x);
        println!("*box_ref_x: {:?}", *box_ref_x);
        println!("&box_ref_x: {:?}", &box_ref_x);

        // 4
        let deref_box_ref_x = *box_ref_x;
        let borrow_box_ref_x = &box_ref_x;
        println!("deref_box_ref_x: {:?}", deref_box_ref_x);
        println!("borrow_box_ref_x: {:?}", borrow_box_ref_x);
    
        // 5
        let box_r_box_y = Box::new(&box_y);
        let box_d_box_y = Box::new(*box_y);
        //let box_box_y = Box::new(box_y);
        //
        // error[E0505]: cannot move out of `box_y` because it is borrowed
        //    --> src\chapter_015.rs:104:34
        //     |
        // 102 |         let box_r_box_y = Box::new(&box_y);
        //     |                                    ------ borrow of `box_y` occurs here
        // 103 |         let box_d_box_y = Box::new(*box_y);
        // 104 |         let box_box_y = Box::new(box_y);
        //     |                                  ^^^^^ move out of `box_y` occurs here
        // 105 |         // let box_box_y_2 = Box::new(box_y); // Use of moved value: box_y value moved above
        // 106 |         println!("box_r_box_y: {:?}", box_r_box_y);
        //     |                                       ----------- borrow later used here
        println!("box_r_box_y: {:?}", box_r_box_y);
        println!("box_d_box_y: {:?}", box_d_box_y);

        // let box_box_y_2 = Box::new(box_y);
        // let _a = *box_box_y_2;
        // println!("box_box_y_2: {:?}", box_box_y_2);
        //         error[E0382]: borrow of moved value: `box_box_y_2`
        //    --> src\chapter_015.rs:122:39
        //     |
        // 121 |         let _a = *box_box_y_2;
        //     |                  ------------ value moved here
        // 122 |         println!("box_box_y_2: {:?}", box_box_y_2);
        //     |                                       ^^^^^^^^^^^ value borrowed here after move
        //     |
        //     = note: move occurs because `*box_box_y_2` has type `Box<i32>`, which does not implement the `Copy` trait
        //     = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
    }

    // 15.2
    {
        use std::ops::{Deref, DerefMut};

        #[derive(Debug)]
        struct MyBox<T>(T);

        impl <T> MyBox <T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        impl <T> Deref for MyBox <T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl <T> DerefMut for MyBox <T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        // Defining Out Own Smart Pointer
        {
            let x = 5;
            let my_box_x = MyBox::new(x);
            println!("x: {}", x);
            println!("my_box_x: {:?}", my_box_x);
            // Without: impl <T> Deref for MyBox <T> {
            // println!("*my_box_x: {:?}", *my_box_x); // Can't be dereferenced

            println!("&my_box_x: {:?}", &my_box_x);
            println!("*my_box_x: {:?}", *my_box_x);
    
            let x_inner_1 = *(my_box_x.deref());
            let x_inner_2 = *my_box_x;
            println!("x_inner_1: {:?}", x_inner_1);
            println!("x_inner_2: {:?}", x_inner_2);
        }

        // Implicit Deref Corcion with Functions and Methods
        {
            #[allow(dead_code)]
            #[derive(Debug)]
            struct InnerU64 {
                inner: u64,
                inner_str: String,
            }

            impl InnerU64 {
                fn new(inner: u64) -> Self {
                    InnerU64 {
                        inner,
                        inner_str: inner.to_string(),
                    }
                }
            }

            impl Deref for InnerU64 {
                type Target = String;
                fn deref(&self) -> &Self::Target {
                    &self.inner_str
                }
            }

            // // TODO: HOWTO???
            // impl Deref for InnerU64 {
            //     type Target = u64;
            //     fn deref(&self) -> &Self::Target {
            //         &self.inner
            //     }
            // }

            fn print_string(s: &str) {
                println!("print_string: s: {:?}", s);
            }

            let slice_s = "abcdefghijklmnopqrstuvwxyz";
            let string_s = String::from("String Value");
            let my_box_slice = MyBox::new("another string slice");
            let my_inner_64 = InnerU64::new(1_024_u64);
            let my_box_string = MyBox::new(String::from("Another String Value"));

            print_string(slice_s);
            print_string(&string_s);
            print_string(&my_box_slice);
            print_string(&my_box_string);
            print_string(&my_inner_64);
            
            // DerefMut
            let mut my_box = MyBox::new("prev".to_string());
            println!("(1) my_box: {:?}", *my_box);
            let new_str = "new string".to_string();
            *my_box = "some random string".to_string();
            println!("(2) my_box: {:?}", *my_box);
            *my_box = new_str;
            println!("(3) my_box: {:?}", *my_box);

            // fn change_t<T>(some_t: &mut T, new_value: T) {
            //     *some_t = new_value;
            // }
            // let mut my_box_s = MyBox::new("a".to_string());
            // let new_s = "New String".to_string();
            // change_t(&mut my_box_s, new_s.clone());
        }
    }
}

fn chapter_015_3() {
    println!("15.3. Running Code on Cleanup with the Drop Trait");

    // ...
    {
        println!("Level 1 +");
        #[derive(Debug)]
        struct MyCustomSmartPointer {
            inner: String,
        }

        impl MyCustomSmartPointer {
            fn new(s: String) -> Self {
                Self { inner: s }
            }
        }

        impl Drop for MyCustomSmartPointer {
            fn drop(&mut self) {
                println!("Dropping MyCustomSmartPointer: {:?}", self.inner);
            }
        }

        {
            let p1 = MyCustomSmartPointer::new("Level 1".to_owned());
            println!("p1: {:?}", p1);
            {
                println!("Level 2 +");
                let p2_1 = MyCustomSmartPointer::new("Level 2 - 1".to_owned());
                let p2_2 = MyCustomSmartPointer::new("Level 2 - 2".to_owned());
                println!("p2_1: {:?}", p2_1);
                println!("p2_2: {:?}", p2_2);
                {
                    println!("Level 3 +");
                    let p3_1 = MyCustomSmartPointer::new("Level 3 - 1".to_owned());
                    println!("p3_1: {:?}", p3_1);
                    println!("Level 3 -");
                } // Drop calls here...
                println!("Level 2 -");
            } // Drop calls here...

            println!("Level 2.2 - about to enter");
            {
                println!("Level 2.2 - entered");
                let p2_2 = MyCustomSmartPointer::new("Level 2.2".to_string());
                println!("p2_2: {:?}", p2_2);
                
                println!("Level 2.2 - about to exit");
            } // Drop calls here...
            println!("Level 2.2 - exited");

            println!("Level 1 -");
        } // Drop calls here...

        // Dropping a Value Early with std::mem::drop
        {
            println!("Testing std::mem::drop +");

            let p1 = MyCustomSmartPointer::new("Level 1".to_owned());
            println!("p1: {:?}", p1);
            drop(p1);
            {
                println!("Level 2 +");
                let p2_1 = MyCustomSmartPointer::new("Level 2 - 1".to_owned());
                let p2_2 = MyCustomSmartPointer::new("Level 2 - 2".to_owned());
                println!("p2_1: {:?}", p2_1);
                println!("p2_2: {:?}", p2_2);
                {
                    println!("Level 3 +");
                    let p3_1 = MyCustomSmartPointer::new("Level 3 - 1".to_owned());
                    println!("p3_1: {:?}", p3_1);
                    println!("Level 3 -");
                } // Drop calls here...
                drop(p2_1);
                println!("Level 2 -");
            } // Drop calls here...

            println!("Testing std::mem::drop -");

            // Testing wrapped smart pointers
            {
                use crate::common::chapter_015::ch_15_2::MyBox;

                let box_wraps_mybox = Box::new(MyBox::new(100));
                println!("1 - box_wraps_box: {:?}", box_wraps_mybox);
                std::mem::drop(box_wraps_mybox);

                // // Error: borrow of moved value: `box_wraps_mybox`
                // println!("2 - box_wraps_box: {:?}", box_wraps_mybox);

                let mybox_wraps_mybox = MyBox::new(MyBox::new(7000));
                println!("2 - mybox_wraps_box: {:?}", mybox_wraps_mybox);
                std::mem::drop(mybox_wraps_mybox);

                let mybox_outter = MyBox::new("outter");
                {
                    let mybox_wraps_mybox = MyBox::new(MyBox::new(7000));
                    println!("3 - mybox_wraps_box: {:?}", mybox_wraps_mybox);

                    let mybox_inner = MyBox::new(&mybox_outter);
                    println!("4 - mybox_inner: {:?}", mybox_inner);
                }

                println!("mybox_outter: {:?}", mybox_outter);
            }
        }
    } // Drop calls here...
}

fn chapter_015_4() {
    println!("15.4. Rc<T>, the Reference Counted Smart Pointer");

    // Common Scope
    {
        #[derive(Debug)]
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }
        
        // Using Rc<T> to Share Data
        {
            use List::{Cons, Nil};

            let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
            let b = Rc::new(Cons(3, Rc::clone(&a)));
            let c = Cons(4, Rc::clone(&a));
            let d = Cons(6, Rc::clone(&b));
            
            println!("a: {:?}", a);
            println!("b: {:?}", b);
            println!("c: {:?}", c);
            println!("d: {:?}", d);
        }

        // Cloning an Rc<T> Increases the Reference Count
        {
            use List::{Cons, Nil};

            let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
            println!("a.count: {:?}, {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
            
            let _b = Rc::new(Cons(3, Rc::clone(&a)));
            println!("created b: a.count: {:?}, {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
            {
                let _c = Cons(4, Rc::clone(&a));
                println!("created c: a.count: {:?}, {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
            }
            println!("destroed c: a.count: {:?}, {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
        }

        // Call std::mem::drop on a Rc-smart-pointer
        {
            use crate::common::chapter_015::ch_15_2::MyBox;
            
            let rc_mybox_1 = Rc::new(MyBox::new(1024));
            let rc_mybox_2 = Rc::clone(&rc_mybox_1);
            let rc_mybox_3 = Rc::clone(&rc_mybox_1);
            
            println!("rc_mybox_1: strong_count: {:?}", Rc::strong_count(&rc_mybox_1));
            println!("rc_mybox_2: strong_count: {:?}", Rc::strong_count(&rc_mybox_2));
            println!("rc_mybox_3: strong_count: {:?}", Rc::strong_count(&rc_mybox_3));
            
            std::mem::drop(rc_mybox_3);
            
            println!("rc_mybox_1: strong_count: {:?}", Rc::strong_count(&rc_mybox_1));
            println!("rc_mybox_2: strong_count: {:?}", Rc::strong_count(&rc_mybox_2));
            // // Error: borrow of moved value
            // println!("rc_mybox_3: strong_count: {:?}", Rc::strong_count(&rc_mybox_3));
        }

        // 
        {
            use crate::common::chapter_015::ch_15_2::MyBox;

            let rc_mybox_1 = Rc::new(MyBox::new(555));
            let rc_mybox_2 = Rc::clone(&rc_mybox_1);
            let rc_mybox_3 = Rc::clone(&rc_mybox_2);
            let rc_mybox_4 = Rc::clone(&rc_mybox_3);
            
            println!("rc_mybox_1: strong_count: {:?}", Rc::strong_count(&rc_mybox_1));
            println!("rc_mybox_2: strong_count: {:?}", Rc::strong_count(&rc_mybox_2));
            println!("rc_mybox_3: strong_count: {:?}", Rc::strong_count(&rc_mybox_3));
            println!("rc_mybox_4: strong_count: {:?}", Rc::strong_count(&rc_mybox_4));

            assert_eq!(4, Rc::strong_count(&rc_mybox_1));
            assert_eq!(4, Rc::strong_count(&rc_mybox_2));
            assert_eq!(4, Rc::strong_count(&rc_mybox_3));
            assert_eq!(4, Rc::strong_count(&rc_mybox_4));
        }
    }

}

fn chapter_015_5() {
    println!("15.5. RefCell<T> and the Interior Mutability Pattern");

    // Common
    {
        // A Use Case for Interior Mutability: Mock Objects
        {
            pub trait Messenger {
                fn send(&self, msg: &str);
            }
    
            #[derive(Debug)]
            pub struct LimitTracker<'a, T: Messenger> {
                messenger: &'a T,
                value: usize,
                max: usize,
            }
    
            impl <'a, T> LimitTracker<'a, T>
            where
                T: Messenger,
            {
                pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
                    LimitTracker {
                        messenger,
                        value: 0,
                        max
                    }
                }
    
                pub fn set_value(mut self, value: usize) {
                    self.value = value;
                    let percentage_of_max = self.value as f64 / self.max as f64;
                    if percentage_of_max >= 1.0 {
                        self.messenger.send("Error: You are over your quota")
                    } else if percentage_of_max >= 0.9 {
                        self.messenger.send("Urgent: 90% reached")
                    } else if percentage_of_max > 0.75 {
                        self.messenger.send("Warning: 75% reached")
                    }
                }
            }
    
            //
            //
            //
    
            #[derive(Debug)]
            struct MockMessenger {
                // OK:
                send_messages: RefCell<Vec<String>>,
    
                // Calling with this...
                //
                // send_messages: Vec<String>,
                //
                // Generates the following output:
                //
                // error[E0596]: cannot borrow `self.send_messages` as mutable, as it is behind a `&` reference
                //    --> src\chapter_015.rs:426:17
                //    |
                // 376 |             fn send(&self, msg: &str);
                //    |                     ----- help: consider changing that to be a mutable reference: `&mut self`
                // ...
                // 426 |                 self.send_messages.push(String::from(msg));
                //    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    
                // warning: variable does not need to be mutable
                //   --> src\chapter_015.rs:432:17
                //    |
                // 432 |             let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
                //    |                 ----^^^^^^^^^^^^^
                //    |                 |
                //    |                 help: remove this `mut`
                //    |
                //    = note: `#[warn(unused_mut)]` on by default
    
                // For more information about this error, try `rustc --explain E0596`.
                // warning: `learn-rust-book` (bin "learn-rust-book") generated 1 warning
                // error: could not compile `learn-rust-book` due to previous error; 1 warning emitted
            }
    
            impl MockMessenger {
                fn new() -> MockMessenger {
                    MockMessenger { send_messages: RefCell::new(vec![]) }
                }
            }
            
            impl Messenger for MockMessenger {
                fn send(&self, msg: &str) {
                    let mut vec_of_strings = self.send_messages.borrow_mut(); 
                    vec_of_strings.push(String::from(msg));
                }
            }
    
            fn it_send_over_75_percent() {
                let mock_messenger = MockMessenger::new();
                let mut _limit_tracker = LimitTracker::new(&mock_messenger, 100);
                _limit_tracker.set_value(80);
                // _limit_tracker has been moved
                println!("mock_messenger: {:?}", mock_messenger);
            }
    
            it_send_over_75_percent();
    
            // #[cfg(test)]
            // mod tests {
            //     #[test]
            //     fn test_over_75 () {
            //         assert_eq!(1, 1);
            //     }
            // }
        }

        // Keeping Track of Borrows at Runtime with RefCell<T>
        {
            #[derive(Debug)]
            struct SomeObject {
                data: RefCell<String>,
            }

            let immutable_object = SomeObject {
                data: RefCell::new(String::from("String Data"))
            };

            // Panic
            {
                // let mut bm_1 = immutable_object.data.borrow_mut();
                // println!("bm_1: {:?}", bm_1);
                // let mut bm_2 = immutable_object.data.borrow_mut();
                // thread 'main' panicked at 'already borrowed: BorrowMutError', src\chapter_015.rs:495:50
                // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
                // error: process didn't exit successfully: `target\debug\learn-rust-book.exe` (exit code: 101)
            }

            // Non-panic
            #[allow(unused_mut)]
            {
                let mut bm_1 = immutable_object.data.try_borrow_mut();
                // OK
                println!("bm_1: {:?}", bm_1);
                let mut bm_2 = immutable_object.data.try_borrow_mut();
                // Err
                println!("bm_2: {:?}", bm_2);

                let mut bm_3 = immutable_object.data.try_borrow_mut();
                match bm_3 {
                    Ok(rm_value) => println!("rm_value: {:?}", rm_value),
                    Err(err) => println!("err: {:?}", err),
                }
            }

            let b_1 = immutable_object.data.borrow();
            let b_2 = immutable_object.data.borrow();
            let b_3 = immutable_object.data.borrow();
            println!("b_1, b_2, b_3: {:?}", (b_1, b_2, b_3));
            println!("immutable_object: {:?}", immutable_object)
        }

        // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
        {
            let value = Rc::new(RefCell::new(100));
            println!("value: {:?}", value);

            let value_2 = Rc::clone(&value);
            let value_3 = Rc::clone(&value);
            let value_4 = Rc::clone(&value);

            println!("(before edit) value: {:?}, value_2: {:?}, value_3: {:?}, value_4: {:?}",
                value, value_2, value_3, value_4);
            
            // 1. Modify the value, ref_mut_value will live as long as the scope is active
            {
                let mut ref_mut_value = value.borrow_mut();
                *ref_mut_value = 200;
                println!("(after edit - 1) value: {:?}, value_2: {:?}, value_3: {:?}, value_4: {:?}",
                    value, value_2, value_3, value_4);
                // Output: 
                // (before edit) value_2: RefCell { value: 100 }, value_3: RefCell { value: 100 }, value_4: RefCell { value: 100 }
                // (after edit) value_2: RefCell { value: <borrowed> }, value_3: RefCell { value: <borrowed> }, value_4: RefCell { value: <borrowed> }
            }
            
            
            // *value.borrow_mut() = 500;
            // is equal of scoped code:
            {
                let mut mr = value.borrow_mut();
                *mr = 505;
                println!("(after edit - 2) value: {:?}, value_2: {:?}, value_3: {:?}, value_4: {:?}",
                value, value_2, value_3, value_4);
            }

            // 2. Modify the value, scoped
            *value.borrow_mut() = 500;
            println!("(after edit - 3) value: {:?}, value_2: {:?}, value_3: {:?}, value_4: {:?}",
                value, value_2, value_3, value_4);
        }
    }
}

fn chapter_015_6() {
    println!("15.6. Reference Cycles Can Leak Memory");

    // Creating a Reference Cycle
    {
        use crate::common::chapter_015::recursive_types::ch_15_6::List::{Cons, Nil};
        
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count: {}", Rc::strong_count(&a));
        println!("a next item: {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation: {}", Rc::strong_count(&a));
        println!("b initial rc count: {}", Rc::strong_count(&b));
        println!("b next item: {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after chaning a = {}", Rc::strong_count(&b));
        println!("a rc count after chaning a = {}", Rc::strong_count(&a));

        // Calling this:
        //
        // println!("a next item = {:?}", a.tail());
        //
        // Generates the following output:
        //
        // a next item = Some(RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, 
        //     RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: 
        //     ...
        //     Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell {
        //     thread 'main' has overflowed its stack
    }

    // Creating a Tree Data Structure: a Node with Child Nodes
    #[allow(dead_code)]
    {
        #[derive(Debug)]
        struct Node {
            value: i32,
            // children: Vec<Node>,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }
        let leaf = Rc::new(Node { 
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        let branch = Rc::new(Node {
            value: 2,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        match leaf.parent.borrow().upgrade() {
            Some(value) => println!("leaf parent: {:?}", value),
            None => println!("leaf has been destroyed"),
        }
        
        println!("{:?}", leaf.parent.borrow().upgrade());
    }

    // Visualizing Changes to strong_count and weak_count
    #[allow(dead_code)]
    {
        #[derive(Debug)]
        struct Node {
            value: i32,
            // children: Vec<Node>,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        let leaf = Rc::new(Node { 
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("<<<Visualizing Changes to strong_count and weak_count>>>");
        println!("leaf strong = {:?}, weak = {:?}, node = {:?}",
            Rc::strong_count(&leaf), Rc::weak_count(&leaf), leaf);
        
        {
            let branch = Rc::new(Node {
                value: 2,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });
            
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            
            match leaf.parent.borrow().upgrade() {
                Some(value) => println!("leaf parent: {:?}", value),
                None => println!("leaf has been destroyed"),
            }
            
            println!("leaf strong = {:?}, weak = {:?}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
            println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::strong_count(&branch));
        }
        
        println!("leaf.parent: {:?}", leaf.parent.borrow().upgrade());
        println!("leaf strong = {:?}, weak = {:?}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
}

fn debuging_struct_that_holds_reference() {
    let s = Rc::new(RefCell::new(String::from("12345")));

    #[derive(Debug)]
    struct A {
        data: Rc<RefCell<String>>,
    }

    let a = A {
        data: Rc::clone(&s),
    };

    println!("s: {:?}", s);
    println!("a: {:?}", a);
    
    s.borrow_mut().push_str(", added to s");
    
    println!("s: {:?}", s);
    println!("a: {:?}", a);
    
    a.data.borrow_mut().push_str(", add a.data");
    
    println!("s: {:?}", s);
    println!("a: {:?}", a);
}
