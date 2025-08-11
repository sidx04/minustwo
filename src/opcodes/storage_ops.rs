use primitive_types::U256;

use crate::{errors::Error, execution::ExecutionContext};

use super::Opcode;

pub fn execute_storage(op: Opcode, ctx: &mut ExecutionContext) -> Result<(), Error> {
    let stack = &mut ctx.machine.stack;

    let res = match op {
        Opcode::SLOAD => {
            let key = stack.pop()?;
            ctx.state.storage.load(key)?;
        }
        Opcode::SSTORE => {
            let key = stack.pop()?;
            let value = stack.pop()?;
            ctx.state.storage.store(key, value);
        }
        _ => todo!(),
    };
    ctx.gas_meter.charge(100)?;

    ctx.machine.pc += 1;
    Ok(())
}
