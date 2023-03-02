//! add_one library crate
use rand::Rng;
use std::ops::RangeInclusive;

/// Adds one to the argument and returns result
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Returns random number withing a rang from `min` to `max`
pub fn get_rand_from_range(min: isize, max: isize) -> isize {
    let range = RangeInclusive::new(min, max);
    rand::thread_rng().gen_range(range)
}

/// Unit tests for add_one crate
#[cfg(test)]
mod tests {
    use super::*;

    /// Unit-test for add_one function
    #[test]
    fn test_add_one() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }

    /// Unit-test for get_rand_from_range function
    #[test]
    fn test_get_rand_from_range() {
        let result = get_rand_from_range(0, 10);
        let is_ok = result >= 0 && result < 10;
        assert!(is_ok);
    }
}
