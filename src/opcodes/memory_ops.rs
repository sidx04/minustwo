use crate::{
    errors::{Error, MemoryError},
    machine::Machine,
};

use super::Opcode;

pub fn execute_memory(op: Opcode, machine: &mut Machine) -> Result<(), Error> {
    let memory = &mut machine.memory;
    let stack = &mut machine.stack;

    let res = match op {
        Opcode::MLOAD => {
            let element = stack.pop().unwrap();
            let byte = element.byte(0);
            if !(1..32).contains(&byte) {
                return Err(Error::MemoryError(MemoryError::InvalidMemoryAccess {
                    offset: byte as usize,
                }));
            }
        }
        Opcode::MSTORE => {
            // Example:
            // PUSH32 0xFF
            // PUSH1 0x00
            // MSTORE
            //  We store 0xFF at memory location 0 but since MSTORE writes 32 bytes,
            // it will store this as 32 bytes, right-aligned — the rest will be zeros.
            // 0x00000000000000000000000000000000000000000000000000000000000000FF
            let offset = stack.pop().unwrap();
            let offset_byte = offset.byte(0);

            if !(1..32).contains(&offset_byte) {
                return Err(Error::MemoryError(MemoryError::InvalidMemoryAccess {
                    offset: offset_byte as usize,
                }));
            }

            let data = stack.pop().unwrap();
            let value: [u8; 32] = data.to_big_endian();

            memory.store(offset_byte.into(), &value).unwrap();
        }
        Opcode::MSIZE => {
            // memory.effective_len(),
            todo!()
        }
        _ => todo!(),
    };
    Ok(())
}
