pub use self::gas::GasError;
pub use self::memory::MemoryError;
pub use self::opcode::OpcodeError;
pub use self::stack::StackError;

pub mod gas;
pub mod memory;
pub mod opcode;
pub mod stack;

use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    StackError(StackError),
    MemoryError(MemoryError),
    GasError(GasError),
    OpcodeError(OpcodeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::StackError(e) => write!(f, "Stack error: {}", e),
            Error::MemoryError(e) => write!(f, "Memory error: {}", e),
            Error::OpcodeError(e) => write!(f, "Opcode error: {}", e),
            Error::GasError(e) => write!(f, "Opcode error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<StackError> for Error {
    fn from(err: StackError) -> Self {
        Error::StackError(err)
    }
}

impl From<MemoryError> for Error {
    fn from(err: MemoryError) -> Self {
        Error::MemoryError(err)
    }
}

impl From<OpcodeError> for Error {
    fn from(err: OpcodeError) -> Self {
        Error::OpcodeError(err)
    }
}

impl From<GasError> for Error {
    fn from(err: GasError) -> Self {
        Error::GasError(err)
    }
}
