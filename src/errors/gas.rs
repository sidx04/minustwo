use std::fmt;

#[derive(Debug, Clone)]
pub enum GasError {
    OutOfGas,
}

impl fmt::Display for GasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GasError::OutOfGas => write!(f, "Out of gas!"),
        }
    }
}
