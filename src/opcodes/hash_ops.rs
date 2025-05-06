use primitive_types::U256;

use crate::{errors::Error, execution::ExecutionContext, utils::hash_keccak};

use super::Opcode;

pub fn execute_keccak256(_op: Opcode, ctx: &mut ExecutionContext) -> Result<(), Error> {
    let stack = &mut ctx.machine.stack;
    let memory = &mut ctx.machine.memory;

    let offset = stack.pop()?;
    let size = stack.pop()?;

    let mut data = memory.access(offset.as_usize(), size.as_usize())?;
    let output = hash_keccak(&mut data)?;
    stack.push(U256::from_big_endian(&output))?;

    ctx.machine.pc += 1;

    Ok(())
}
