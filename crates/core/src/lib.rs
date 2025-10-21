/// core: small utilities used across the workspace
use regex::Regex;

/// simple function to check if text contains a pattern
pub fn contains_pattern(text: &str, pattern: &str) -> Result<bool, regex::Error> {
    let re = Regex::new(pattern)?;
    Ok(re.is_match(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_pattern() {
        assert_eq!(contains_pattern("hello 123", r"\d+").unwrap(), true);
        assert_eq!(contains_pattern("no digits", r"\d+").unwrap(), false);
    }
}
