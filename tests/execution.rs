use minustwo::utils::setup;
use primitive_types::U256;

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    const MEM_SIZE: usize = 64;

    #[test]
    fn test_execution_context1() {
        // [`Opcode`]: [PUSH1 1, PUSH1 10, ADD]
        let code = vec![0x60, 0x01, 0x60, 0x0A, 0x01];
        let mut ctx = setup(vec![], code, 21000, MEM_SIZE);

        ctx.run().unwrap();

        assert_eq!(ctx.machine.stack.pop().unwrap(), U256::from(11));
        assert!(ctx.machine.stack.is_empty());
    }

    #[test]
    fn test_execution_context2() {
        // [`Opcode`]: [PUSH1 10, PUSH1 10, PUSH1 8, ADDMOD, PUSH1 4, EQ]
        let code = vec![0x60, 0x0A, 0x60, 0x0A, 0x60, 0x08, 0x08, 0x60, 0x04, 0x14];
        let mut ctx = setup(vec![], code, 21000, MEM_SIZE);

        ctx.run().unwrap();

        assert_eq!(ctx.machine.stack.pop().unwrap(), U256::from(1));
        assert!(ctx.machine.stack.is_empty());
    }

    #[test]
    fn test_execution_context3() {
        // [`Opcode`]: [PUSH1 10, PUSH1 2, EXP, STOP]
        let code = vec![0x60, 0x0A, 0x60, 0x02, 0x0A, 0x00, 0x0A];

        let mut ctx = setup(vec![], code, 21000, MEM_SIZE);

        ctx.run().unwrap();

        // assert_eq!(machine.stack.pop().unwrap(), U256::from(0x64)); // 10 ** 2 = 100
        assert_eq!(ctx.machine.stack.is_empty(), true);
    }

    #[test]
    #[should_panic]
    fn test_execution_panic() {
        // [`Opcode`]: [PUSH1 10, STOP, PUSH1 20, PUSH1 10]
        let code = vec![0x60, 0x0A, 0x00, 0x60, 0x14, 0x60, 0x0A];

        let mut ctx = setup(vec![], code, 21000, MEM_SIZE);

        ctx.run().unwrap();

        // ideally, the `machine` should halt after encountering `STOP`
        // so `PC` should be ideally 2 and not 4 and top of stack should be 10, not 20
        let popped_val = ctx.machine.stack.pop().unwrap();
        assert_ne!(popped_val, U256::from(0x14));
        assert_eq!(popped_val, U256::from(0x0A));
        assert_eq!(ctx.machine.pc, 2);
    }

    #[test]
    fn test_memory_codes() {
        // [`Opcode`]: [PUSH1 FF, PUSH1 00, MSTORE, PUSH1 FF, PUSH1 01, MSTORE, MLOAD]
        let code = vec![0x60, 0xFF, 0x60, 0x00, 0x52, 0x60, 0xFF, 0x60, 0x01, 0x52];

        let mut ctx = setup(vec![], code, 21000, MEM_SIZE);

        ctx.run().unwrap();

        println!("{}", ctx.machine.memory);
    }

    #[test]
    fn test_env_opcodes() {
        // [`Opcode`] : [ADDRESS, BALANCE, CALLER, BALANCE]
        let code = vec![0x30, 0x31, 0x33, 0x031];
        let mut ctx = setup(vec![], code, 21000, MEM_SIZE);
        ctx.run().unwrap();

        assert_eq!(ctx.machine.stack.pop().unwrap(), U256::from(1000));
        assert_eq!(ctx.machine.stack.pop().unwrap(), U256::from(100));

        // [`Opcode`] : [PUSH1 0, CALLDATALOAD, PUSH1 2, CALLDATALOAD, CALLDATASIZE]
        let code = vec![0x60, 0x00, 0x35, 0x60, 0x02, 0x35, 0x36];
        let mut ctx = setup(vec![], code, 21000, MEM_SIZE);
        ctx.run().unwrap();

        assert_eq!(ctx.machine.stack.pop().unwrap(), U256::from(9));
        assert_eq!(
            ctx.machine.stack.pop().unwrap(),
            U256::from_str("0xAABBCCDD123456").unwrap()
        );
        assert_eq!(
            ctx.machine.stack.pop().unwrap(),
            U256::from_str("0xFFEEAABBCCDD123456").unwrap()
        );

        // [`Opcode`] : [PUSH1 32, PUSH1 0, PUSH1 0, CALLDATACOPY, PUSH1 8, PUSH1 2, PUSH1 0,
        //              CALLDATACOPY]
        let code = vec![
            0x60, 0x20, 0x60, 0x00, 0x60, 0x00, 0x37, 0x60, 0x08, 0x60, 0x02, 0x60, 0x00, 0x37,
        ];
        let mut ctx = setup(vec![], code, 21000, MEM_SIZE);
        ctx.run().unwrap();

        println!("{}", ctx.machine.memory);
    }

    #[test]
    fn test_hash() {
        // [`Opcode`] : [PUSH1 0xFF, PUSH1 0, MSTORE, PUSH1 4, PUSH1 0, KECCAK256]
        let code = vec![0x61, 0xFF, 0x61, 0x00, 0x52, 0x61, 0x04, 0x061, 0x00, 0x20];
        let mut ctx = setup(vec![], code, 21000, MEM_SIZE);
        ctx.run().unwrap();

        assert_eq!(
            ctx.machine.stack.pop().unwrap(),
            U256::from_str("e8e77626586f73b955364c7b4bbf0bb7f7685ebd40e852b164633a4acbd3244c")
                .unwrap()
        );
    }
}
