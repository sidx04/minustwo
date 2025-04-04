use crate::constants::MAX_STACK_DEPTH;
use crate::errors::memory::StackError;
use arrayvec::ArrayVec;
use primitive_types::U256;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Stack {
    contents: ArrayVec<U256, MAX_STACK_DEPTH>,
}

impl Stack {
    pub fn init() -> Self {
        Stack {
            contents: ArrayVec::new(),
        }
    }

    pub fn push(&mut self, element: U256) -> Result<(), StackError> {
        if element >= (U256::max_value()) {
            return Err(StackError::InvalidItem);
        }

        if self.contents.len() >= MAX_STACK_DEPTH {
            return Err(StackError::StackOverflow);
        }

        self.contents.push(element);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<U256, StackError> {
        self.contents.pop().ok_or(StackError::EmptyStack)
    }

    pub fn peek(&self) -> Result<&U256, StackError> {
        self.contents.last().ok_or(StackError::EmptyStack)
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn clear(&mut self) {
        self.contents.clear();
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Stack: ")?;
        for (i, element) in self.contents.iter().enumerate().rev() {
            writeln!(f, "  [{}]: 0x{:02X}", i, element)?;
        }
        Ok(())
    }
}
