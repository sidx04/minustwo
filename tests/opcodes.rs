use minustwo::opcodes::{Opcode, arithmetic::execute_arithmetic};
use minustwo::{machine::Machine, opcodes::stack_ops::execute_stack};
use primitive_types::U256;
use std::rc::Rc;
mod tests {

    use minustwo::opcodes::stack_ops::execute_stack_duplicate;

    use super::*;

    #[test]
    fn test_push1() {
        let code = Rc::new(vec![0x60, 0x0A]);
        let mut machine = Machine::new(code, Rc::new(vec![]), 1024);

        execute_stack(Opcode::PUSH1, 1, &mut machine).unwrap();
        assert_eq!(machine.stack.pop().unwrap(), U256::from(10));
        assert_eq!(machine.pc, 2);
    }

    #[test]
    fn test_push8() {
        let code = Rc::new(vec![0x67, 0xFFFFFFFFFFFFFFFF]);
        let mut machine = Machine::new(code, Rc::new(vec![]), 1024);

        execute_stack(Opcode::PUSH8, 8, &mut machine).unwrap();
        // assert_eq!(machine.stack.pop().unwrap(), U256::from(10));
        // assert_eq!(machine.pc, 2);
    }

    #[test]
    fn test_duplicate() {
        let code = Rc::new(vec![0x01, 0x00, 0x00]);
        let mut machine = Machine::new(code.clone(), Rc::new(vec![]), 1024);

        let a = code[0];
        let b = code[1];
        let c = code[2];

        machine.stack.push(U256::from(a)).unwrap();
        machine.stack.push(U256::from(b)).unwrap();
        machine.stack.push(U256::from(c)).unwrap();

        println!("Before duplicating:\n{}", machine.stack);

        execute_stack_duplicate(Opcode::DUP3, 3, &mut machine).unwrap();

        println!("After duplicating:\n{}", machine.stack);
    }

    #[test]
    #[should_panic]
    fn test_duplicate_panic() {
        let code = Rc::new(vec![0x01, 0x00, 0x00]);
        let mut machine = Machine::new(code.clone(), Rc::new(vec![]), 1024);

        let a = code[0];
        let b = code[1];

        machine.stack.push(U256::from(a)).unwrap();
        machine.stack.push(U256::from(b)).unwrap();

        println!("Before duplicating:\n{}", machine.stack);

        execute_stack_duplicate(Opcode::DUP3, 3, &mut machine).unwrap();

        println!("After duplicating:\n{}", machine.stack);
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
