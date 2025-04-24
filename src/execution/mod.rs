pub mod execute;
pub mod gas;

use crate::machine::{Machine, state::AccountState};
use execute::execute_opcode;
use gas::GasMeter;
use primitive_types::{H160, U256};

#[derive(Debug)]
pub struct ExecutionContext<'a> {
    pub machine: &'a mut Machine,
    pub state: &'a mut AccountState,
    pub caller: H160,
    pub callee: H160,
    pub value: U256,
    pub gas_meter: GasMeter,
    pub input_data: Vec<usize>,
}

impl<'a> ExecutionContext<'a> {
    pub fn new(
        machine: &'a mut Machine,
        state: &'a mut AccountState,
        caller: H160,
        callee: H160,
        value: U256,
        gas_meter: GasMeter,
        input_data: Vec<usize>,
    ) -> Self {
        Self {
            machine,
            state,
            caller,
            callee,
            value,
            gas_meter,
            input_data,
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.machine.pc >= self.machine.code.len() {
            return Err("Execution halted: Reached end of code".to_string());
        }

        let opcode = self.machine.code[self.machine.pc];
        println!("Opcode: {opcode:02X}");

        execute_opcode(self, opcode).unwrap();

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.machine.pc < self.machine.code.len() {
            if self.machine.halted {
                break;
            }
            self.step()?;
        }
        println!(
            "Gas used: {} | Gas remaining: {}",
            self.gas_meter.used(),
            self.gas_meter.remaining()
        );
        Ok(())
    }
}
