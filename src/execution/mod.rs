pub mod execute;
pub mod gas;

use crate::machine::{Machine, state::State};
use execute::execute_opcode;
use gas::GasMeter;
use primitive_types::{H160, U256};

#[derive(Debug)]
pub struct ExecutionContext<'a> {
    pub machine: &'a mut Machine,
    pub state: &'a mut State,
    pub caller: H160,
    pub callee: H160,
    pub value: U256,
    pub gas_meter: GasMeter,
    pub calldata: Vec<u8>,
}

impl<'a> ExecutionContext<'a> {
    pub fn new(
        machine: &'a mut Machine,
        state: &'a mut State,
        caller: H160,
        callee: H160,
        value: U256,
        gas_meter: GasMeter,
        calldata: Vec<u8>,
    ) -> Self {
        Self {
            machine,
            state,
            caller,
            callee,
            value,
            gas_meter,
            calldata,
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
