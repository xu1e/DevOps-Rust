/// automated_testing: examples that replace pytest demos with Rust unit tests
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::contains_pattern;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_contains_pattern_from_core() {
        assert!(contains_pattern("abc123", r"\d+").unwrap());
    }
}
