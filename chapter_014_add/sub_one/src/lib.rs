//! sub_one library crate

/// Subtracts one from the argument and returns the result.
pub fn sub_one(i: i32) -> i32 {
    i - 1
}

/// sub_one tests module
#[cfg(test)]
mod tests {
    use super::*;

    /// Unit test for the `sub_one` function, 1
    #[test]
    /// Unit test for the `sub_one` function, 2
    fn test_sub_one() {
        let result = sub_one(10);
        assert_eq!(result, 9);
    }
}
