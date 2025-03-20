use std::fmt;

#[derive(Debug, Clone)]
pub enum StackError {
    InvalidItem,
    StackOverflow,
    EmptyStack,
}

#[derive(Debug, Clone)]
pub enum MemoryError {
    InvalidMemoryAccess { offset: usize },
    MemoryLimitExceeded { attempted: usize, limit: usize },
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackError::InvalidItem => write!(f, "Item is out of 256-bit bound!"),
            StackError::StackOverflow => write!(f, "Stack overflow: Maximum depth reached!"),
            StackError::EmptyStack => write!(f, "Stack underflow: Stack is empty!"),
        }
    }
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
