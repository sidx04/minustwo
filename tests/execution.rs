use minustwo::{execution::ExecutionContext, machine::Machine, machine::state::AccountState};
use primitive_types::U256;
use std::rc::Rc;
#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use primitive_types::H160;

    use super::*;

    #[test]
    fn test_execution_context1() {
        let mut state = AccountState {
            accounts: HashMap::new(),
        };

        // [`Opcode`]: [PUSH1 1, PUSH1 10, ADD, STOP]
        let code: Vec<usize> = vec![0x60, 0x01, 0x60, 0x0A, 0x01, 0x00];

        let caller = H160::from_low_u64_be(0xabc);
        let callee = H160::from_low_u64_be(0xdef);

        state.create_account(caller, U256::from(1000), vec![]);
        state.create_account(callee, U256::zero(), code);

        let code = Rc::new(state.get_account(&callee).unwrap().code.clone());
        let mut machine = Machine::new(code, Rc::new(vec![]), 1024);

        let mut ctx = ExecutionContext {
            machine: &mut machine,
            state: &mut state,
            caller,
            callee,
            value: U256::zero(),
            input_data: vec![],
        };
        ctx.run().unwrap();

        assert_eq!(machine.stack.pop().unwrap(), U256::from(0x0B)); // 1 + 10 = 11
    }

    #[test]
    fn test_execution_context2() {
        let mut state = AccountState {
            accounts: HashMap::new(),
        };
        // [`Opcode`]: [PUSH1 10, PUSH1 10, PUSH1 10, PUSH1 8, ADDMOD, PUSH1 4, EQ, STOP]
        let code = vec![
            0x60, 0x0A, 0x60, 0x0A, 0x60, 0x08, 0x08, 0x60, 0x04, 0x14, 0x00,
        ];
        let caller = H160::from_low_u64_be(0xabc);
        let callee = H160::from_low_u64_be(0xdef);

        state.create_account(caller, U256::from(1000), vec![]);
        state.create_account(callee, U256::zero(), code);

        let code = Rc::new(state.get_account(&callee).unwrap().code.clone());
        let mut machine = Machine::new(code, Rc::new(vec![]), 1024);

        let mut ctx = ExecutionContext {
            machine: &mut machine,
            state: &mut state,
            caller,
            callee,
            value: U256::zero(),
            input_data: vec![],
        };
        ctx.run().unwrap();

        assert_eq!(machine.stack.pop().unwrap(), U256::from(0x01)); // (10 + 10) % 8 = 4; 4 == 4 ? = 1
        assert_eq!(machine.stack.is_empty(), true);
    }

    // #[test]
    // fn test_execution_context3() {
    //     // [`Opcode`]: [PUSH1 10, PUSH1 2, EXP, STOP]
    //     let code = Rc::new(vec![0x60, 0x0A, 0x60, 0x02, 0x0A, 0x00, 0x0A]);
    //     let mut machine = Machine::new(code, Rc::new(vec![]), 1024);
    //     println!("{:02X?}", machine);

    //     let mut ctx = ExecutionContext::new(&mut machine);

    //     ctx.run().unwrap();

    //     assert_eq!(machine.stack.pop().unwrap(), U256::from(0x64)); // 10 ** 2 = 100
    //     assert_eq!(machine.stack.is_empty(), true);
    // }

    #[test]
    fn test_execution_panic() {
        let mut state = AccountState {
            accounts: HashMap::new(),
        };
        // [`Opcode`]: [PUSH1 10, STOP, PUSH1 20, PUSH1 10]
        let code = vec![0x60, 0x0A, 0x00, 0x60, 0x14, 0x60, 0x0A];

        let caller = H160::from_low_u64_be(0xabc);
        let callee = H160::from_low_u64_be(0xdef);

        state.create_account(caller, U256::from(1000), vec![]);
        state.create_account(callee, U256::zero(), code);

        let code = Rc::new(state.get_account(&callee).unwrap().code.clone());
        let mut machine = Machine::new(code, Rc::new(vec![]), 1024);

        let mut ctx = ExecutionContext {
            machine: &mut machine,
            state: &mut state,
            caller,
            callee,
            value: U256::zero(),
            input_data: vec![],
        };
        ctx.run().unwrap();

        // ideally, the `machine` should halt after encountering `STOP`
        // so `PC` should be ideally 2 and not 4 and top of stack should be 10, not 20
        let popped_val = machine.stack.pop().unwrap();
        assert_ne!(popped_val, U256::from(0x14));
        assert_eq!(popped_val, U256::from(0x0A));
        assert_eq!(machine.pc, 2);
    }
}
