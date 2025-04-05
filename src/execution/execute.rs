use crate::errors::stack::StackError;
use crate::machine::Machine;
use crate::opcodes::{
    Opcode,
    arithmetic::{execute_arithmetic, execute_logical},
    control::execute_control,
    stack_ops::execute_stack,
};

pub fn execute_opcode(machine: &mut Machine, opcode: u8) -> Result<(), StackError> {
    match opcode {
        0x00 => execute_control(Opcode::STOP, machine),
        0x01 => execute_arithmetic(Opcode::ADD, machine),
        0x02 => execute_arithmetic(Opcode::SUB, machine),
        0x03 => execute_arithmetic(Opcode::MUL, machine),
        0x04 => execute_arithmetic(Opcode::DIV, machine),
        0x05 => execute_arithmetic(Opcode::SDIV, machine),
        0x06 => execute_arithmetic(Opcode::MOD, machine),
        0x07 => execute_arithmetic(Opcode::SMOD, machine),
        0x08 => execute_arithmetic(Opcode::ADDMOD, machine),
        0x09 => execute_arithmetic(Opcode::MULMOD, machine),
        0x0A => execute_arithmetic(Opcode::EXP, machine),
        0x0B => execute_arithmetic(Opcode::SIGNEXTEND, machine),
        0x10 => execute_logical(Opcode::LT, machine),
        0x11 => execute_logical(Opcode::SLT, machine),
        0x12 => execute_logical(Opcode::GT, machine),
        0x13 => execute_logical(Opcode::SGT, machine),
        0x14 => execute_logical(Opcode::EQ, machine),
        0x15 => execute_logical(Opcode::ISZERO, machine),
        0x16 => execute_logical(Opcode::AND, machine),
        0x17 => execute_logical(Opcode::OR, machine),
        0x18 => execute_logical(Opcode::XOR, machine),
        0x19 => execute_logical(Opcode::NOT, machine),
        0x60 => execute_stack(Opcode::PUSH1, machine),

        _ => Err(StackError::InvalidItem), // Handle unknown opcodes
    }?;
    println!("After executing {:02X?}:\n{}", opcode, machine.stack);
    Ok(())
}
