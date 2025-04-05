use minustwo::opcodes::{Opcode, arithmetic::execute_arithmetic};
use minustwo::{machine::Machine, opcodes::stack_ops::execute_stack};
use primitive_types::U256;
use std::rc::Rc;
mod tests {

    use super::*;

    #[test]
    fn test_push1() {
        let code = Rc::new(vec![0x60, 0x0A]);
        let mut machine = Machine::new(code, Rc::new(vec![]), 1024);

        execute_stack(Opcode::PUSH1, &mut machine).unwrap();
        assert_eq!(machine.stack.pop().unwrap(), U256::from(10));
        assert_eq!(machine.pc, 2);
    }

    #[test]
    fn test_add() {
        let code = Rc::new(vec![0x10, 0x0A]);
        let mut machine = Machine::new(code.clone(), Rc::new(vec![]), 1024);
        let a = code[0];
        let b = code[1];
        machine.stack.push(U256::from(a)).unwrap();
        machine.stack.push(U256::from(b)).unwrap();

        execute_arithmetic(Opcode::ADD, &mut machine).unwrap();

        assert_eq!(machine.stack.pop().unwrap(), U256::from(0x1A));
    }
}
