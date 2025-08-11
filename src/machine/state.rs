use primitive_types::{H160, U256};
use std::collections::HashMap;

use crate::storage::kv::KeyValueStore;

#[derive(Debug, Clone)]
pub struct Account {
    pub nonce: U256,
    pub balance: U256,
    pub code: Vec<usize>,
    pub address: H160,
}

#[derive(Debug)]
pub struct State {
    // contract_address -> its account
    pub accounts: HashMap<H160, Account>,
    // contract_address -> its storage, for now let it be K-V store
    pub storage: KeyValueStore,
    // pub snapshots: Vec<(HashMap<H160, Account>, HashMap<H160, KeyValueStore>)> || todo()!
}

impl State {
    pub fn get_account(&self, address: &H160) -> Option<&Account> {
        self.accounts.get(address)
    }

    pub fn get_account_mut(&mut self, address: &H160) -> Option<&mut Account> {
        self.accounts.get_mut(address)
    }

    pub fn create_account(&mut self, address: H160, balance: U256, code: Vec<usize>) {
        self.accounts.insert(
            address,
            Account {
                address,
                nonce: U256::zero(),
                balance,
                code,
            },
        );
    }
}
