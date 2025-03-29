use crate::errors::memory::StackError;
use crate::machine::Machine;
use crate::opcodes::{Opcode, arithmetic::execute_arithmetic, stack_ops::execute_push1};

pub fn execute_opcode(machine: &mut Machine, opcode: u8) -> Result<(), StackError> {
    match opcode {
        0x60 => execute_push1(machine),                   // PUSH1
        0x01 => execute_arithmetic(Opcode::ADD, machine), // ADD
        _ => Err(StackError::InvalidItem),                // Handle unknown opcodes
    }?;
    println!("After executing {:02X?}: {}", opcode, machine.stack);
    Ok(())
}
