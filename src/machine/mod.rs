use std::rc::Rc;

pub use self::{memory::Memory, stack::Stack};

pub mod memory;
pub mod stack;
pub mod state;

#[derive(Debug, Clone)]
pub struct Machine {
    pub data: Rc<Vec<usize>>,
    pub code: Rc<Vec<usize>>,
    pub memory: Memory,
    pub stack: Stack,
    pub pc: usize,
    pub halted: bool,
    pub invalid: bool,
}

impl Machine {
    pub fn new(code: Rc<Vec<usize>>, data: Rc<Vec<usize>>, memory_limit: usize) -> Self {
        Self {
            data,
            code,
            memory: Memory::init(memory_limit),
            stack: Stack::init(),
            pc: 0,
            halted: false,
            invalid: false,
        }
    }

    pub fn code(&self) -> &[usize] {
        &self.code
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }
}
