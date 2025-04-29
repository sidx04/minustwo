use crate::constants::MAX_STACK_DEPTH;
use crate::errors::stack::StackError;
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

    pub fn get_contents(&self) -> ArrayVec<U256, MAX_STACK_DEPTH> {
        self.contents.clone()
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

    pub fn swap(&mut self, index: usize) -> Result<(), StackError> {
        let contents = &mut self.contents;
        let length = contents.len();
        contents.swap(length - 1, index);
        Ok(())
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
        writeln!(
            f,
            "┌────────────────────────────── Stack ──────────────────────────────┐"
        )?;
        if self.contents.is_empty() {
            writeln!(f, "│         [ empty stack ]       │")?;
        } else {
            for (i, value) in self.contents.iter().rev().enumerate() {
                writeln!(
                    f,
                    "│ [{:02}] │ 0x{:0>56X} │",
                    self.contents.len() - 1 - i,
                    value
                )?;
            }
        }
        writeln!(
            f,
            "└───────────────────────────────────────────────────────────────────┘"
        )
    }
}
