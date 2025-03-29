use minustwo::{execution::ExecutionContext, machine::Machine};
use primitive_types::U256;
use std::rc::Rc;
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_execution_context() {
        let code = Rc::new(vec![0x60, 0x01, 0x60, 0x0A, 0x01]); // PUSH1 1, PUSH1 10, ADD
        let mut machine = Machine::new(code, Rc::new(vec![]), 1024);
        println!("{:02X?}", machine);

        let mut ctx = ExecutionContext::new(&mut machine);

        ctx.run().unwrap();

        assert_eq!(machine.stack.pop().unwrap(), U256::from(0x0B)); // 1 + 10 = 11
    }
}
