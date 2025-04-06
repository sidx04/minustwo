use primitive_types::{H160, U256};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct Account {
    pub nonce: U256,
    pub balance: U256,
    pub storage: BTreeMap<U256, U256>,
    pub code: Vec<usize>,
    pub address: H160,
}

#[derive(Debug)]
pub struct AccountState {
    pub accounts: HashMap<H160, Account>,
}

impl AccountState {
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
                storage: BTreeMap::new(),
                code,
            },
        );
    }
}
