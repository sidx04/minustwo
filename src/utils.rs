use alloy_primitives::Keccak256;
use primitive_types::{H160, U256};
use std::{rc::Rc, str::FromStr};

use crate::{
    errors::Error,
    execution::{ExecutionContext, gas::GasMeter},
    machine::{Machine, state::State},
    storage::kv::KeyValueStore,
};

pub fn setup(
    caller_code: Vec<usize>,
    callee_code: Vec<usize>,
    gas_limit: u64,
    memory_limit: usize,
) -> ExecutionContext<'static> {
    use std::collections::HashMap;

    let mut state = State {
        accounts: HashMap::new(),
        storage: KeyValueStore::new(), // change later to account global state
    };

    let caller = H160::from_str("0xbe862ad9abfe6f22bcb087716c7d89a26051f74c").unwrap();
    let callee = H160::from_str("0x9bbfed6889322e016e0a02ee459d306fc19545d8").unwrap();

    state.create_account(caller, U256::from(1000), caller_code);
    state.create_account(callee, U256::from(100), callee_code.clone());

    let code = Rc::new(callee_code);
    let machine = Box::leak(Box::new(Machine::new(code, Rc::new(vec![]), memory_limit))); // Leak to extend lifetime

    ExecutionContext {
        machine,
        state: Box::leak(Box::new(state)),
        caller,
        callee,
        value: U256::zero(),
        calldata: vec![0xFF, 0xEE, 0xAA, 0xBB, 0xCC, 0xDD, 0x12, 0x34, 0x56],
        gas_meter: GasMeter::new(gas_limit),
    }
}

pub fn hash_keccak(bytes: Vec<u8>) -> Result<[u8; 32], Error> {
    let mut hasher = Keccak256::new();
    let mut output = [0u8; 32];

    hasher.update(bytes);
    hasher.finalize_into_array(&mut output);

    Ok(output)
}
