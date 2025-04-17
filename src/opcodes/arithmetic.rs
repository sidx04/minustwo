use primitive_types::U256;

use crate::errors::{Error, StackError};
use crate::machine::Machine;
use crate::opcodes::Opcode;

pub fn execute_arithmetic(op: Opcode, machine: &mut Machine) -> Result<(), Error> {
    let stack = &mut machine.stack;
    let a = stack.pop()?;
    let b = stack.pop()?;

    let result = match op {
        Opcode::ADD => a.checked_add(b),
        Opcode::SUB => a.checked_sub(b),
        Opcode::MUL => a.checked_mul(b),
        Opcode::DIV => {
            if b == U256::zero() {
                return Err(Error::StackError(StackError::InvalidItem));
            }
            a.checked_div(b)
        }
        Opcode::SDIV => todo!(), // signed division
        Opcode::MOD => a.checked_rem(b),
        Opcode::SMOD => todo!(), // signed mod
        Opcode::ADDMOD => {
            let c = stack.pop()?;
            let inter = (c.checked_add(b)).unwrap();
            inter.checked_rem(a)
        }
        Opcode::MULMOD => {
            let c = stack.pop()?;
            let inter = (c.checked_add(b)).unwrap();
            inter.checked_rem(a)
        }
        Opcode::EXP => {
            println!("{}", b.pow(a));
            Some(b.pow(a))
        }
        Opcode::SIGNEXTEND => todo!(),
        _ => todo!(),
    }
    .unwrap();

    stack.push(result)?;

    machine.pc += 1;

    Ok(())
}

pub fn execute_logical(op: Opcode, machine: &mut Machine) -> Result<(), Error> {
    let stack = &mut machine.stack;
    let a = stack.pop()?;
    let b = stack.pop()?;

    let result = match op {
        Opcode::LT | Opcode::SLT => U256::from((a < b) as usize),
        Opcode::GT | Opcode::SGT => U256::from((a > b) as usize),
        Opcode::EQ => U256::from((a == b) as usize),
        Opcode::ISZERO => U256::from((a == U256::zero()) as usize),
        Opcode::AND => a & b,
        Opcode::OR => a | b,
        Opcode::XOR => a ^ b,
        Opcode::NOT => !a,
        _ => todo!(),
    };

    stack.push(result)?;

    machine.pc += 1;

    Ok(())
}
