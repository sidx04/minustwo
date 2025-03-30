use minustwo::{execution::ExecutionContext, machine::Machine};
use primitive_types::U256;
use std::rc::Rc;
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_execution_context1() {
        // PUSH1 1, PUSH1 10, ADD
        let code = Rc::new(vec![0x60, 0x01, 0x60, 0x0A, 0x01]);
        let mut machine = Machine::new(code, Rc::new(vec![]), 1024);
        println!("{:02X?}", machine);

        let mut ctx = ExecutionContext::new(&mut machine);

        ctx.run().unwrap();

        assert_eq!(machine.stack.pop().unwrap(), U256::from(0x0B)); // 1 + 10 = 11
    }

    #[test]
    fn test_execution_context2() {
        // [`Opcode`]: [PUSH1 10, PUSH1 10, PUSH1 10, PUSH1 8, ADDMOD, PUSH1 4, EQ]
        let code = Rc::new(vec![
            0x60, 0x0A, 0x60, 0x0A, 0x60, 0x08, 0x08, 0x60, 0x04, 0x14,
        ]);
        let mut machine = Machine::new(code, Rc::new(vec![]), 1024);
        println!("{:02X?}", machine);

        let mut ctx = ExecutionContext::new(&mut machine);

        ctx.run().unwrap();

        assert_eq!(machine.stack.pop().unwrap(), U256::from(0x01)); // (10 + 10) % 8 = 4; 4 == 4 ? = 1
    }
}
