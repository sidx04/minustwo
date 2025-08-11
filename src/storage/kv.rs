use primitive_types::U256;
use std::collections::BTreeMap;
use std::fmt;

use crate::errors::{Error, StorageError};

#[derive(Default, Clone, Debug)]
pub struct KeyValueStore {
    data: BTreeMap<U256, U256>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&self, key: U256) -> Result<U256, Error> {
        if !self.data.contains_key(&key) {
            return Err(Error::StorageError(StorageError::InvalidKey));
        }

        match self.data.get(&key) {
            Some(&value) if value != U256::zero() => Ok(value),
            _ => Err(Error::StorageError(StorageError::EmptyKey)),
        }
    }

    pub fn store(&mut self, key: U256, value: U256) {
        self.data.insert(key, value);
    }

    pub fn remove(&mut self, key: U256) {
        self.data.remove(&key);
    }
}

impl fmt::Display for KeyValueStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Storage:\n")?;
        for (key, value) in &self.data {
            writeln!(f, "{} => {}", key, value)?;
        }
        Ok(())
    }
}
