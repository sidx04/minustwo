use primitive_types::U256;

use crate::errors::memory::MemoryError;

#[derive(Clone, Debug)]
pub struct Memory {
    data: Vec<u8>,
    effective_len: U256,
    limit: usize,
}

impl Memory {
    pub fn init(limit: usize) -> Self {
        Self {
            data: Vec::new(),
            effective_len: U256::zero(),
            limit,
        }
    }

    pub fn load(&self, offset: usize) -> Result<u8, MemoryError> {
        // offset cannot be more than limit
        if offset >= self.limit {
            return Err(MemoryError::InvalidMemoryAccess { offset });
        }
        // offset is accessing unwrittem parts of memory
        if offset >= self.data.len() {
            Ok(0)
        } else {
            Ok(self.data[offset])
        }
    }

    pub fn access(&self, offset: usize, size: usize) -> Result<Vec<u8>, MemoryError> {
        // accessing beyond memory limits
        if offset + size > self.limit {
            return Err(MemoryError::InvalidMemoryAccess { offset });
        }

        let mut result = vec![0u8; size];

        if offset < self.data.len() {
            let available = &self.data[offset..self.data.len().min(offset + size)];
            result[..available.len()].copy_from_slice(available);
        }
        Ok(result)
    }

    pub fn store(&mut self, offset: usize, value: &[u8]) -> Result<(), MemoryError> {
        let new_size = offset + value.len();

        // do not store anything beyond memory limit
        if new_size > self.limit {
            return Err(MemoryError::MemoryLimitExceeded {
                attempted: new_size,
                limit: self.limit,
            });
        }

        if new_size > self.data.len() {
            self.data.resize(new_size, 0);
        }
        self.data[offset..offset + value.len()].copy_from_slice(value);
        self.update_effective_len(new_size);

        Ok(())
    }

    pub fn resize(&mut self, new_size: usize) -> Result<(), MemoryError> {
        // resize only <= memory.limit
        if new_size > self.limit {
            return Err(MemoryError::MemoryLimitExceeded {
                attempted: new_size,
                limit: self.limit,
            });
        }
        if new_size > self.data.len() {
            self.data.resize(new_size, 0);
            self.update_effective_len(new_size);
        }
        Ok(())
    }

    pub fn effective_len(&self) -> U256 {
        self.effective_len
    }

    fn update_effective_len(&mut self, new_len: usize) {
        if U256::from(new_len) > self.effective_len {
            self.effective_len = U256::from(new_len);
        }
    }

    // Actual way how the gas cost for a memory expansion is calculated, TBD
    pub fn compute_memory_expansion_cost(&mut self) {
        todo!()
    }
}

use std::fmt;

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Memory {{")?;
        writeln!(f, "  limit: {}", self.limit)?;
        writeln!(f, "  effective_len: {}", self.effective_len)?;
        writeln!(f, "  data ({} bytes):", self.data.len())?;

        const BYTES_PER_LINE: usize = 8; // group by 8 bytes
        for (i, chunk) in self.data.chunks(BYTES_PER_LINE).enumerate() {
            write!(f, "    {:04X}: ", i * BYTES_PER_LINE)?;
            for byte in chunk {
                write!(f, "{:02X} ", byte)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "}}")
    }
}
