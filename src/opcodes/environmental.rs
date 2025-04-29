use primitive_types::{H160, H256, U256};

use crate::{errors::Error, execution::ExecutionContext};

use super::Opcode;

pub fn execute_env(op: Opcode, ctx: &mut ExecutionContext) -> Result<(), Error> {
    let stack = &mut ctx.machine.stack;
    let memory = &mut ctx.machine.memory;
    let _res = match op {
        Opcode::ADDRESS => {
            let address = ctx.callee;
            ctx.machine.stack.push(U256::from_big_endian(&address.0))?;
        }
        Opcode::BALANCE => {
            let addr = ctx.machine.stack.pop()?;
            let addr = H160::from(H256::from(U256::to_big_endian(&addr)));
            let balance = ctx
                .state
                .get_account(&addr)
                .map_or(U256::zero(), |acc| acc.balance);
            ctx.machine.stack.push(balance)?;
        }
        Opcode::CALLER => {
            let caller = ctx.caller;
            ctx.machine.stack.push(U256::from_big_endian(&caller.0))?;
        }
        Opcode::CALLVALUE => {
            ctx.machine.stack.push(ctx.value)?;
        }
        Opcode::CALLDATALOAD => {
            let offset = stack.pop()?;
            let calldata = &ctx.calldata;
            let slice = &calldata[offset.as_usize()..];
            let calldata_slice = U256::from_big_endian(slice);
            stack.push(calldata_slice)?;
        }
        Opcode::CALLDATASIZE => {
            stack.push(U256::from(ctx.calldata.len()))?;
        }
        Opcode::CALLDATACOPY => {
            let dest_offset = stack.pop()?;
            let calldata_offset = stack.pop()?;
            let calldata = &ctx.calldata;
            let slice = &calldata[calldata_offset.as_usize()..];

            memory.store(dest_offset.as_usize(), slice)?;
        }
        _ => todo!(),
    };
    ctx.machine.pc += 1;
    Ok(())
}
