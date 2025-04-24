use primitive_types::U256;

use crate::{errors::Error, execution::ExecutionContext};

use super::Opcode;

pub fn execute_control(op: Opcode, ctx: &mut ExecutionContext) -> Result<(), Error> {
    let stack = &mut ctx.machine.stack;
    let memory = &mut ctx.machine.memory;

    let _ = match op {
        Opcode::STOP | Opcode::RETURN => {
            // let exit_status = match stack.is_empty() {
            //     true => {
            //         println!("Code executed!");
            //         U256::one()
            //     }
            //     false => {
            //         println!("Stack should be empty!");
            //         U256::zero()
            //     }
            // };
            // println!("Machine exited with status: {}", exit_status);
            stack.clear();
            memory.clear();

            ctx.machine.halted = true;
        }
        _ => todo!(),
    };
    ctx.gas_meter.charge(0)?;

    Ok(())
}
