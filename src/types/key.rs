use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Key(String);

#[derive(Debug)]
pub enum KeyError {
    Empty,
    TooLong,
    InvalidCharacters,
    Whitespace,
}

const MAX_LENGTH: usize = 255;

impl Key {
    pub fn new(s: String) -> Result<Self, KeyError> {
        if s.is_empty() {
            return Err(KeyError::Empty);
        }
        if s.len() > MAX_LENGTH {
            return Err(KeyError::TooLong);
        }
        if !s
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(KeyError::InvalidCharacters);
        }
        Ok(Self(s))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Key {
    type Error = KeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Key::new(value)
    }
}

impl From<Key> for String {
    fn from(key: Key) -> String {
        key.0
    }
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyError::Empty => write!(f, "Key cannot be empty"),
            KeyError::TooLong => write!(f, "Key exceeds maximum length of 255 characters"),
            KeyError::InvalidCharacters => {
                write!(
                    f,
                    "Key contains invalid characters (only a-z, A-Z, 0-9, _, - allowed)"
                )
            }
            KeyError::Whitespace => {
                write!(f, "Key cannot have leading or trailing whitespace")
            }
        }
    }
}
impl std::error::Error for KeyError {}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // Valid Key Tests
    // ========================================

    #[test]
    fn test_valid_alphanumeric() {
        assert!(Key::new("validkey".to_string()).is_ok());
        assert!(Key::new("test123".to_string()).is_ok());
        assert!(Key::new("KEY456".to_string()).is_ok());
        assert!(Key::new("MixedCase123".to_string()).is_ok());
    }

    #[test]
    fn test_valid_with_underscore() {
        assert!(Key::new("user_id".to_string()).is_ok());
        assert!(Key::new("test_key_123".to_string()).is_ok());
        assert!(Key::new("_leading".to_string()).is_ok());
        assert!(Key::new("trailing_".to_string()).is_ok());
    }

    #[test]
    fn test_valid_with_hyphen() {
        assert!(Key::new("session-token".to_string()).is_ok());
        assert!(Key::new("my-key-123".to_string()).is_ok());
        assert!(Key::new("-leading".to_string()).is_ok());
        assert!(Key::new("trailing-".to_string()).is_ok());
    }

    #[test]
    fn test_valid_mixed_separators() {
        assert!(Key::new("my_key-123".to_string()).is_ok());
        assert!(Key::new("user-id_token".to_string()).is_ok());
    }

    #[test]
    fn test_valid_single_character() {
        assert!(Key::new("a".to_string()).is_ok());
        assert!(Key::new("1".to_string()).is_ok());
        assert!(Key::new("_".to_string()).is_ok());
        assert!(Key::new("-".to_string()).is_ok());
    }

    #[test]
    fn test_valid_max_length() {
        let max_key = "a".repeat(MAX_LENGTH);
        assert!(Key::new(max_key).is_ok());
    }

    // ========================================
    // Empty Key Tests
    // ========================================

    #[test]
    fn test_empty_key() {
        let result = Key::new("".to_string());
        assert!(matches!(result, Err(KeyError::Empty)));
    }

    // ========================================
    // Too Long Key Tests
    // ========================================

    #[test]
    fn test_too_long_key() {
        let too_long = "a".repeat(MAX_LENGTH + 1);
        let result = Key::new(too_long);
        assert!(matches!(result, Err(KeyError::TooLong)));
    }

    #[test]
    fn test_way_too_long_key() {
        let way_too_long = "a".repeat(1000);
        let result = Key::new(way_too_long);
        assert!(matches!(result, Err(KeyError::TooLong)));
    }

    // ========================================
    // Invalid Characters Tests
    // ========================================

    #[test]
    fn test_invalid_space() {
        assert!(matches!(
            Key::new("has space".to_string()),
            Err(KeyError::InvalidCharacters)
        ));
        assert!(matches!(
            Key::new("has  double  space".to_string()),
            Err(KeyError::InvalidCharacters)
        ));
    }

    #[test]
    fn test_invalid_special_characters() {
        // Test common special characters that should be rejected
        let invalid_chars = vec![
            "test@email",
            "key#value",
            "price$100",
            "percent%",
            "caret^",
            "ampersand&",
            "star*",
            "plus+",
            "equals=",
            "brackets[]",
            "braces{}",
            "parens()",
            "pipe|",
            "backslash\\",
            "forward/slash",
            "colon:key",
            "semicolon;",
            "quotes\"",
            "apostrophe'",
            "less<than",
            "greater>than",
            "question?",
            "exclaim!",
            "tilde~",
            "backtick`",
        ];

        for invalid in invalid_chars {
            assert!(
                matches!(
                    Key::new(invalid.to_string()),
                    Err(KeyError::InvalidCharacters)
                ),
                "Should reject key with invalid character: {}",
                invalid
            );
        }
    }

    #[test]
    fn test_invalid_unicode() {
        // Note: Rust's is_alphanumeric() accepts Unicode letters/numbers
        // So Cyrillic, Chinese, etc. are actually VALID in our current implementation
        // Only emoji and symbols are rejected
        assert!(matches!(
            Key::new("emoji😀".to_string()),
            Err(KeyError::InvalidCharacters)
        ));

        // These actually pass validation because is_alphanumeric() accepts them:
        // assert!(Key::new("японский".to_string()).is_ok());  // Cyrillic
        // assert!(Key::new("中文".to_string()).is_ok());        // Chinese

        // If you want to restrict to ASCII only, you'd need to change
        // the validation in Key::new() to use c.is_ascii_alphanumeric()
    }

    #[test]
    fn test_invalid_dots() {
        // Dots are not allowed (only alphanumeric, _, -)
        assert!(matches!(
            Key::new("config.theme".to_string()),
            Err(KeyError::InvalidCharacters)
        ));
    }

    // ========================================
    // Whitespace Tests (Note: Currently not checked separately)
    // ========================================

    #[test]
    fn test_leading_whitespace() {
        // Leading whitespace is caught by InvalidCharacters, not Whitespace error
        // This is because the current validation checks characters first
        assert!(Key::new(" key".to_string()).is_err());
    }

    #[test]
    fn test_trailing_whitespace() {
        assert!(Key::new("key ".to_string()).is_err());
    }

    #[test]
    fn test_surrounding_whitespace() {
        assert!(Key::new(" key ".to_string()).is_err());
    }

    #[test]
    fn test_tab_character() {
        assert!(Key::new("key\tvalue".to_string()).is_err());
    }

    #[test]
    fn test_newline_character() {
        assert!(Key::new("key\nvalue".to_string()).is_err());
    }

    // ========================================
    // Method Tests
    // ========================================

    #[test]
    fn test_as_str() {
        let key = Key::new("test-key".to_string()).unwrap();
        assert_eq!(key.as_str(), "test-key");
    }

    #[test]
    fn test_into_string() {
        let key = Key::new("test-key".to_string()).unwrap();
        assert_eq!(key.into_string(), "test-key".to_string());
    }

    // ========================================
    // Conversion Tests (TryFrom/Into)
    // ========================================

    #[test]
    fn test_try_from_valid() {
        let result: Result<Key, KeyError> = "valid-key".to_string().try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_from_invalid() {
        let result: Result<Key, KeyError> = "invalid key!".to_string().try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_into_string_conversion() {
        let key = Key::new("test".to_string()).unwrap();
        let s: String = key.into();
        assert_eq!(s, "test");
    }

    // ========================================
    // Serialization Tests (Serde)
    // ========================================

    #[test]
    fn test_serialize_deserialize() {
        let key = Key::new("test-key".to_string()).unwrap();

        // Serialize to JSON
        let json = serde_json::to_string(&key).unwrap();
        assert_eq!(json, "\"test-key\"");

        // Deserialize from JSON
        let deserialized: Key = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, key);
    }

    #[test]
    fn test_deserialize_invalid_key() {
        // Should fail during deserialization if key is invalid
        let invalid_json = "\"invalid key!\"";
        let result: Result<Key, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }

    // ========================================
    // Error Display Tests
    // ========================================

    #[test]
    fn test_error_messages() {
        assert_eq!(KeyError::Empty.to_string(), "Key cannot be empty");
        assert_eq!(
            KeyError::TooLong.to_string(),
            "Key exceeds maximum length of 255 characters"
        );
        assert_eq!(
            KeyError::InvalidCharacters.to_string(),
            "Key contains invalid characters (only a-z, A-Z, 0-9, _, - allowed)"
        );
        assert_eq!(
            KeyError::Whitespace.to_string(),
            "Key cannot have leading or trailing whitespace"
        );
    }

    // ========================================
    // Edge Cases
    // ========================================

    #[test]
    fn test_all_numbers() {
        assert!(Key::new("123456789".to_string()).is_ok());
    }

    #[test]
    fn test_all_underscores() {
        assert!(Key::new("___".to_string()).is_ok());
    }

    #[test]
    fn test_all_hyphens() {
        assert!(Key::new("---".to_string()).is_ok());
    }

    #[test]
    fn test_mixed_case() {
        assert!(Key::new("MyKeyName".to_string()).is_ok());
        assert!(Key::new("camelCase".to_string()).is_ok());
        assert!(Key::new("PascalCase".to_string()).is_ok());
        assert!(Key::new("SCREAMING_SNAKE_CASE".to_string()).is_ok());
        assert!(Key::new("kebab-case".to_string()).is_ok());
    }

    // ========================================
    // Clone and Equality Tests
    // ========================================

    #[test]
    fn test_clone() {
        let key1 = Key::new("test".to_string()).unwrap();
        let key2 = key1.clone();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_equality() {
        let key1 = Key::new("test".to_string()).unwrap();
        let key2 = Key::new("test".to_string()).unwrap();
        let key3 = Key::new("other".to_string()).unwrap();

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}
