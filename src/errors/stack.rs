use std::fmt;

#[derive(Debug, Clone)]
pub enum StackError {
    InvalidItem,
    StackOverflow,
    EmptyStack,
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
