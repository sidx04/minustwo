use std::fmt;

#[derive(Debug, Clone)]
pub enum OpcodeError {
    InvalidOpcode(usize),
    NotImplemented(String),
    ExecutionFailure(String),
}

impl fmt::Display for OpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpcodeError::InvalidOpcode(op) => write!(f, "Invalid opcode: 0x{:02X}", op),
            OpcodeError::NotImplemented(name) => write!(f, "Opcode not implemented: {}", name),
            OpcodeError::ExecutionFailure(msg) => write!(f, "Opcode execution failed: {}", msg),
        }
    }
}

impl std::error::Error for OpcodeError {}
