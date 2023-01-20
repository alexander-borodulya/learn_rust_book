pub mod ch_15_2 {

    use std::{ops::{Deref, DerefMut}, fmt::Debug};

    #[derive(Debug)]
    pub struct MyBox<T>(T)
    where T: 
        std::fmt::Debug
    ;

    #[allow(dead_code)]
    impl <T> MyBox <T>
    where T:
        std::fmt::Debug
    {
        pub fn new(x: T) -> MyBox<T> {
            println!("MyBox<T>: new: {:?}", x);
            MyBox(x)
        }
    }

    impl <T> Deref for MyBox <T>
    where T:
        std::fmt::Debug
    {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl <T> DerefMut for MyBox <T>
    where T:
        std::fmt::Debug
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl <T> Drop for MyBox <T>
    where T:
        std::fmt::Debug
    {
        fn drop(&mut self) {
            println!("MyBox: drop: {:p} - {:?}", self, self);
        }
    }
}

pub mod recursive_types {


    pub mod ch_15_6 {
        
        use std::{cell::RefCell, rc::Rc};
        use List::{Cons, Nil};
        
        #[allow(dead_code)]
        #[derive(Debug)]
        pub enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }
        
        #[allow(dead_code)]
        impl List {
            pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Cons(_, item) => Some(item),
                    Nil => None,
                }
            }
        }
    }
}
