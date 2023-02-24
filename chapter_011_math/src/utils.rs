/// sub, left - right
pub fn sub(left: isize, right: isize) -> isize {
    left - right
}

/// Internal unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }
}
