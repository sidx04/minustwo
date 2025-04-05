use std::fmt;

#[derive(Debug, Clone)]
pub enum MemoryError {
    InvalidMemoryAccess { offset: usize },
    MemoryLimitExceeded { attempted: usize, limit: usize },
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::InvalidMemoryAccess { offset } => {
                write!(f, "{}", format!("Invalid memory access at offset {offset}"))
            }
            MemoryError::MemoryLimitExceeded { attempted, limit } => {
                write!(
                    f,
                    "{}",
                    format!("Memory limit exceeded: attempted {attempted}, limit {limit}")
                )
            }
        }
    }
}
