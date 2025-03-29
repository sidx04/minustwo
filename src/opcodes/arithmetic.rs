use primitive_types::U256;

use crate::errors::memory::StackError;
use crate::machine::Machine;
use crate::opcodes::Opcode;

pub fn execute_arithmetic(op: Opcode, machine: &mut Machine) -> Result<(), StackError> {
    let stack = &mut machine.stack;
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    let result = match op {
        Opcode::ADD => a.checked_add(b),
        Opcode::SUB => a.checked_sub(b),
        Opcode::MUL => a.checked_mul(b),
        Opcode::DIV => {
            if b == U256::zero() {
                return Err(StackError::InvalidItem);
            }
            a.checked_div(b)
        }
        _ => todo!(),
    }
    .unwrap();

    stack.push(result)?;

    machine.pc += 1;

    Ok(())
}
