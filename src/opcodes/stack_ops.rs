use super::Opcode;
use crate::{errors::stack::StackError, machine::Machine};
use primitive_types::U256;

pub fn execute_stack_push(
    _op: Opcode,
    _offset: usize,
    machine: &mut Machine,
) -> Result<(), StackError> {
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

pub fn execute_stack_duplicate(
    _op: Opcode,
    index: usize,
    machine: &mut Machine,
) -> Result<(), StackError> {
    let stack = &mut machine.stack;

    let contents = stack.get_contents();

    let dup_index = contents.iter().enumerate().len() - index;
    println!("{}", dup_index);

    if dup_index > stack.get_contents().len() {
        return Err(StackError::InvalidItem);
    }

    let value = contents.get(dup_index).ok_or(StackError::EmptyStack)?;

    stack.push(*value)?;

    machine.pc += index;

    Ok(())
}
