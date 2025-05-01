use alloy_primitives::Keccak256;
use primitive_types::U256;

use crate::{errors::Error, execution::ExecutionContext};

use super::Opcode;

pub fn execute_keccak256(_op: Opcode, ctx: &mut ExecutionContext) -> Result<(), Error> {
    let stack = &mut ctx.machine.stack;
    let memory = &mut ctx.machine.memory;
    let pc = &mut ctx.machine.pc;
    let mut hasher = Keccak256::new();

    let offset = stack.pop()?;
    let size = stack.pop()?;

    let mut data = memory.access(offset.as_usize(), size.as_usize())?;

    hasher.update(&mut data);

    let mut output = [0u8; 32];
    hasher.finalize_into_array(&mut output);

    stack.push(U256::from_big_endian(&output))?;

    *pc += 1;

    Ok(())
}
