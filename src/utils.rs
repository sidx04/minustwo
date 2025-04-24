use std::rc::Rc;

use crate::{
    execution::{ExecutionContext, gas::GasMeter},
    machine::{Machine, state::AccountState},
};

pub fn setup(
    caller_code: Vec<usize>,
    callee_code: Vec<usize>,
    gas_limit: u64,
    memory_limit: usize,
) -> ExecutionContext<'static> {
    use primitive_types::{H160, U256};
    use std::collections::HashMap;

    let mut state = AccountState {
        accounts: HashMap::new(),
    };

    let caller = H160::from_low_u64_be(0xabc);
    let callee = H160::from_low_u64_be(0xdef);

    state.create_account(caller, U256::from(1000), caller_code);
    state.create_account(callee, U256::from(1), callee_code.clone());

    let code = Rc::new(callee_code);
    let machine = Box::leak(Box::new(Machine::new(code, Rc::new(vec![]), memory_limit))); // Leak to extend lifetime

    ExecutionContext {
        machine,
        state: Box::leak(Box::new(state)),
        caller,
        callee,
        value: U256::zero(),
        input_data: vec![],
        gas_meter: GasMeter::new(gas_limit),
    }
}
