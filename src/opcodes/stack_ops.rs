use crate::{errors::memory::StackError, machine::Machine};
use primitive_types::U256;

pub fn execute_push1(machine: &mut Machine) -> Result<(), StackError> {
    let pc = machine.pc;

    if pc + 1 >= machine.code.len() {
        return Err(StackError::InvalidItem);
    }

    let value = machine.code[pc + 1];

    machine.stack.push(U256::from(value))?;

    println!("{}", machine.stack);

    // increment PC to skip the operand (1 byte)
    machine.pc += 2;

    Ok(())
}
