use primitive_types::U256;

use crate::{errors::memory::StackError, machine::Machine};

use super::Opcode;

pub fn execute_control(op: Opcode, machine: &mut Machine) -> Result<(), StackError> {
    let stack = &mut machine.stack;
    let _ = match op {
        Opcode::STOP => {
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
            machine.halted = true;
        }
        _ => todo!(),
    };
    Ok(())
}
