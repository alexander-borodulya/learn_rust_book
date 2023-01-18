#![allow(dead_code)]

////////////////////////////////////////////////////////////////////////////////
///
/// Chapter 7 - Begin
/// 

pub mod rust_book;

mod fron_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

///
/// Chapter 7 - End
/// 
////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////
///
/// Chapter 11 - Begin
///

// 1. Declare internal module
pub mod chapter_011_math;

// 2. Import module content / Reuse module content
pub use chapter_011_math::{add, utils::sub};

pub fn mult(left: isize, right: isize) -> isize {
    left * right
}

///
/// Chapter 11 - End
///
////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////
///
/// Chapter 12 - Begin
///

pub mod chapter_012_lib;
pub mod chapter_013_lib;

///
/// Chapter 12 - End
///
////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////
///
/// Library crate unit-tests
/// 

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_sub() {
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mult() {
        assert_eq!(mult(100, 500), 50000);
    }
}

//
// Library crate unit-tests - End
//
////////////////////////////////////////////////////////////////////////////////
