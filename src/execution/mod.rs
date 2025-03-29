pub mod execute;

use crate::machine::Machine;
use execute::execute_opcode;

#[derive(Debug)]
pub struct ExecutionContext<'a> {
    machine: &'a mut Machine,
}

impl<'a> ExecutionContext<'a> {
    pub fn new(machine: &'a mut Machine) -> Self {
        Self { machine }
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.machine.pc >= self.machine.code.len() {
            return Err("Execution halted: Reached end of code".to_string());
        }

        let opcode = self.machine.code[self.machine.pc];
        println!("Opcode: {opcode:02X}");

        execute_opcode(self.machine, opcode).unwrap();

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.machine.pc < self.machine.code.len() {
            self.step()?;
        }
        Ok(())
    }
}
