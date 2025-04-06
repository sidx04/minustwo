use memory::MemoryError;
use stack::StackError;

pub mod memory;
pub mod stack;

#[derive(Debug, Clone)]
pub enum Error {
    StackError(StackError),
    MemoryError(MemoryError),
}
