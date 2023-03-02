#[allow(dead_code)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[allow(dead_code)]
pub fn get_5() -> i32 {
    5
}

#[allow(dead_code)]
pub fn get_five() -> &'static str {
    "five"
}

//
// Testing Equality with the assert_eq! and assert_ne! Macros
//
#[allow(dead_code)]
fn add_two(i: i32) -> i32 {
    i + 2
}

#[allow(dead_code)]
fn add_two_wbug(i: i32) -> i32 {
    // OK: i + 2
    i + 3
}

//
// Adding Custom Failure Messages
//
#[allow(dead_code)]
fn greeting(name: String) -> String {
    format!("Hello, {}!", name)
}

#[allow(dead_code)]
fn greeting_wbug(_name: String) -> String {
    String::from("Hello, {}!")
}

#[cfg(test)]
mod tests_chapter_011_1 {
    use super::*;
    use crate::common::chapter_005::Rectangle;
    use crate::common::chapter_011::{Guess, MyError, Object};

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn test_get_five_and_5() {
        let r1 = get_5();
        let r2 = get_five();
        assert_eq!(r1, 5);
        assert_eq!(r2, "five");
    }

    //
    // Checking Results with the assert! Macro
    //

    #[test]
    fn test_larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };

        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        let r = larger.can_hold(&smaller);
        assert!(r);
    }

    #[test]
    fn test_larger_can_hold_smaller_wbug() {
        let larger = Rectangle::new(10, 10);
        let smaller = Rectangle::new(5, 5);
        let r = larger.can_hold_wbug(&smaller);
        assert!(r);
    }

    #[test]
    fn test_smaller_cannot_hold_larger() {
        let larger = Rectangle::new(10, 10);
        let smaller = Rectangle::new(5, 5);
        let r = smaller.can_hold(&larger);
        assert!(!r);
    }

    //
    // Testing Equality with the assert_eq! and assert_ne! Macros
    //

    #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn test_add_two_wbug() {
        assert_eq!(4, add_two_wbug(2));
    }

    #[test]
    fn test_assert_ne() {
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn test_assert_eq_with_struct() {
        let r1 = Rectangle {
            width: 5,
            height: 5,
        };

        let r2 = Rectangle {
            width: 5,
            height: 5,
        };

        assert_eq!(r1, r2);
    }

    #[test]
    fn test_assert_ne_with_struct() {
        let r1 = Rectangle {
            width: 50,
            height: 5,
        };

        let r2 = Rectangle {
            width: 5,
            height: 5,
        };

        assert_ne!(r1, r2);
    }

    //
    // Adding Custom Failure Messages
    //

    #[test]
    fn test_greeting_contains_name() {
        let result = greeting("Carol".to_string());
        assert!(result.contains("Carol"));

        let result = greeting_wbug("Carol".to_string());
        assert!(
            result.contains("Carol"),
            "Greeting should contain name, result: {:?}",
            result
        );
    }

    #[test]
    fn test_assert_format_message() {
        assert!(false, "ASSERT FORMAT MSG: assert! failed! {}", false);
    }

    #[test]
    fn test_assert_eq_format_message() {
        let l = 1;
        let r = 2;
        assert_eq!(l, r, "ASSERT FORMAT MSG: assert_eq! failed! {:?}", (l, r));
    }

    #[test]
    fn test_assert_ne_format_message() {
        let l = 15;
        let r = 15;
        assert_ne!(l, r, "ASSERT FORMAT MSG: assert_ne! failed!: {:?}", (l, r));
    }

    //
    // Checking for Panics with should_panic
    //
    #[test]
    #[should_panic]
    fn test_guess_gt100() {
        Guess::new(1001);
    }

    #[test]
    #[should_panic]
    fn test_guess_gt100_wbug() {
        Guess::new_wbug(1001);
    }

    #[test]
    #[should_panic(expected = "must be less than 100")]
    fn test_guess_gt100_with_expected() {
        Guess::new(1001);
    }

    #[test]
    #[should_panic(expected = "must be less than 100")]
    fn test_guess_gt100_wbug_panic_msg() {
        Guess::new_wbug_panic_msg(1001);
    }

    #[test]
    #[should_panic(expected = "panic expected")]
    fn test_object_expected_panic() {
        Object::new(0);
    }

    #[test]
    #[should_panic(expected = "panic expected")]
    fn test_object_unexpected_panic() {
        Object::new(1);
    }

    #[test]
    #[should_panic(expected = "panic expected")]
    fn test_object_never_panics() {
        Object::new(2);
    }

    //
    // Using Result<T, E> in Tests
    //
    #[test]
    fn test_add_two_result() -> Result<(), String> {
        let r1 = add_two(8);
        if 10 == r1 {
            Ok(())
        } else {
            Err(String::from(format!(
                "add_two failed: Expected 10 got: {}",
                r1
            )))
        }
    }

    #[test]
    fn test_add_two_result_failed() -> Result<(), String> {
        let r1 = add_two(10);
        if 10 == r1 {
            Ok(())
        } else {
            Err(String::from(format!(
                "add_two failed: Expected 10 got: {}",
                r1
            )))
        }
    }

    fn failed_if_ten(x: i32) -> Result<(), String> {
        if x == 10 {
            Err(String::from("X is 10, Failed!"))
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_body_with_result_001() -> Result<(), String> {
        failed_if_ten(1)
    }

    #[test]
    fn test_body_with_result_002() -> Result<(), String> {
        failed_if_ten(10)
    }

    #[test]
    fn test_body_with_result_003() -> Result<(), MyError> {
        Err(MyError {
            message: String::from("Failed!"),
        })
    }

    #[test]
    fn test_body_with_result_004() -> Result<(), String> {
        Err(String::from("Error message!"))
    }
}

//
// chapter_011_2 - Begin
//
#[cfg(test)]
mod tests_chapter_011_2 {

    #[test]
    fn test_always_ok() {
        println!(">>> The test_always_ok always passes <<<");
        assert_eq!(true, true, "Always ok");
    }

    #[test]
    fn test_always_ok_ch11_2_001() {
        assert_eq!(true, true, "Always ok: (test_always_ok_ch11_2_001)");
    }

    #[test]
    #[ignore]
    fn test_always_ok_ch11_2_002() {
        assert_eq!(true, true, "Always ok: (test_always_ok_ch11_2_002)");
    }

    #[test]
    #[ignore]
    fn test_always_ok_ch11_2_003() {
        assert_eq!(true, true, "Always ok: (test_always_ok_ch11_2_003)");
    }
}
