use std::rc::Rc;

pub use self::{memory::Memory, stack::Stack};

pub mod memory;
pub mod stack;
pub mod state;
pub mod storage;

#[derive(Debug, Clone)]
pub struct Machine {
    pub data: Rc<Vec<u8>>,
    pub code: Rc<Vec<u8>>,
    pub memory: Memory,
    pub stack: Stack,
    pub pc: usize,
}

impl Machine {
    pub fn new(code: Rc<Vec<u8>>, data: Rc<Vec<u8>>, memory_limit: usize) -> Self {
        Self {
            data,
            code,
            memory: Memory::init(memory_limit),
            stack: Stack::init(),
            pc: 0,
        }
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }
}
