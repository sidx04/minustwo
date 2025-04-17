use primitive_types::U256;

use crate::{errors::Error, machine::Machine};

use super::Opcode;

pub fn execute_control(op: Opcode, machine: &mut Machine) -> Result<(), Error> {
    let stack = &mut machine.stack;
    let memory = &mut machine.memory;

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

            machine.halted = true;
        }
        _ => todo!(),
    };
    Ok(())
}
