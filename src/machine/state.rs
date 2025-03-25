use crate::execution::context::ExecutionContext;

#[derive(Debug, Clone)]
pub struct State {
    pub pc: usize,
    pub call_depth: usize,
    pub halted: bool,
    pub execution_context: ExecutionContext,
}
