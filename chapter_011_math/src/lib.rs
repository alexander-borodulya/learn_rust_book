pub mod utils;

pub fn add(left: isize, right: isize) -> isize {
    left + right
}

pub use utils::sub;

pub fn mult(left: isize, right: isize) -> isize {
    left * right
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
