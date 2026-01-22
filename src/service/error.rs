use crate::types::Key;

#[derive(Debug, Clone, PartialEq)]
pub enum StorageError {
    KeyNotFound(Key),
    KeyAlreadyExists(Key),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::KeyNotFound(key) => {
                write!(f, "The key '{}' does not exist in the store", key.as_str())
            }
            StorageError::KeyAlreadyExists(key) => {
                write!(f, "The key '{}' already exists in the store", key.as_str())
            }
        }
    }
}

impl std::error::Error for StorageError {}

impl StorageError {
    pub fn error_code(&self) -> &'static str {
        match self {
            StorageError::KeyNotFound(_) => "KEY_NOT_FOUND",
            StorageError::KeyAlreadyExists(_) => "KEY_ALREADY_EXISTS",
        }
    }
}
