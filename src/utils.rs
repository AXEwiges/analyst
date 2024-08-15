use anyhow::Result;
use std::str::FromStr;

/// Attempts to parse a string into a given type.
pub fn parse_value<T: FromStr>(value: &str) -> Result<T>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    value.parse::<T>().map_err(|e| anyhow::anyhow!("Failed to parse value: {}", e))
}

/// Calculates the percentage and formats it as a string.
pub fn percentage(part: usize, total: usize) -> String {
    if total == 0 {
        "0.00%".to_string()
    } else {
        format!("{:.2}%", (part as f64 / total as f64) * 100.0)
    }
}

/// Truncates a string to a maximum length, adding an ellipsis if truncated.
pub fn truncate(s: &str, max_chars: usize) -> String {
    if s.len() <= max_chars {
        s.to_string()
    } else {
        format!("{}...", &s[..max_chars])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value::<i32>("42").unwrap(), 42);
        assert!(parse_value::<i32>("not a number").is_err());
    }

    #[test]
    fn test_percentage() {
        assert_eq!(percentage(25, 100), "25.00%");
        assert_eq!(percentage(0, 100), "0.00%");
        assert_eq!(percentage(50, 0), "0.00%");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello, world!", 5), "Hello...");
        assert_eq!(truncate("Short", 10), "Short");
    }
}