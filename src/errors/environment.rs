use std::fmt;

#[derive(Debug, Clone)]
pub enum EnvError {
    AddressNotFound,
    InvalidBalance,
    InvalidOrigin,
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvError::AddressNotFound => write!(f, "Given address is not a 20-byte address!"),
            EnvError::InvalidBalance => write!(f, "Balance cannot be negative!"),
            EnvError::InvalidOrigin => write!(f, "Address cannot be Origin, it contains code!"),
        }
    }
}
