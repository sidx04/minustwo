use crate::errors::Error;
use crate::errors::GasError::OutOfGas;

#[derive(Debug)]
pub struct GasMeter {
    pub gas_limit: u64,
    gas_used: u64,
}

impl GasMeter {
    pub fn new(limit: u64) -> Self {
        Self {
            gas_limit: limit,
            gas_used: 0,
        }
    }

    pub fn used(&self) -> u64 {
        self.gas_used
    }

    pub fn charge(&mut self, amount: u64) -> Result<(), Error> {
        if self.gas_used + amount > self.gas_limit {
            return Err(Error::GasError(OutOfGas));
        }
        self.gas_used += amount;
        Ok(())
    }

    pub fn remaining(&self) -> u64 {
        self.gas_limit - self.gas_used
    }
}
