use std::fmt;

#[derive(Debug, Clone)]
pub enum StorageError {
    InvalidKey,
    EmptyKey,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::InvalidKey => write!(f, "Key does not exist!"),
            StorageError::EmptyKey => write!(f, "There is no corresponding value for this key!"),
        }
    }
}
