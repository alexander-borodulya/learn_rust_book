//
// Import functions from the chapter_011_math library crate
// 

use chapter_011_math::add;
use chapter_011_math::sub;
use chapter_011_math::mult;

/// Runs payload test
pub fn run() {
    println!("11. Writing Automated Tests");
    chapter_011_3();
}

//
// Binary crate functionality that re-use library functions.
// 
pub fn add_sub_mult(left: isize, right: isize) -> isize {
    let p1 = add(left, right);
    let p2 = sub(left, right);
    let p3 = mult(p1, p2);
    p3
}

fn chapter_011_3() {
    println!("11.3. Test Organization");
    
    // [Some Payload] Use functions from the library crate
    {
        let i1 = add(100, 25);
        let i2 = sub(100, 25);
        println!("i1 = {}, i2 = {}", i1, i2);
    
        let i3 = add_sub_mult(100, 25);
        println!("i3 = {}", i3);
    }
}

//
// Binary crate unit tests.
// 
#[cfg(test)]
mod tests_chapter_011_3 {
    use super::*;

    #[test]
    fn test_add_sub_mult() {
        let r = add_sub_mult(1000, 100);
        assert_eq!(r, 990000)
    }
}
