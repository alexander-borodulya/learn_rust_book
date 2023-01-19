pub mod utils;

pub fn add(left: isize, right: isize) -> isize {
    left + right
}

/// Chapter 11 - unit-tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
