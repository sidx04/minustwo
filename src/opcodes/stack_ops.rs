use super::Opcode;
use crate::{
    errors::{Error, StackError},
    execution::ExecutionContext,
};
use primitive_types::U256;

pub fn execute_stack_push(
    _op: Opcode,
    _offset: usize,
    ctx: &mut ExecutionContext,
) -> Result<(), Error> {
    let pc = ctx.machine.pc;

    if pc + 1 >= ctx.machine.code.len() {
        return Err(Error::StackError(StackError::InvalidItem));
    }

    let value = ctx.machine.code[pc + 1];

    ctx.machine.stack.push(U256::from(value))?;

    println!("{}", ctx.machine.stack);

    // increment PC to skip the operand (1 byte)
    ctx.machine.pc += 2;

    ctx.gas_meter.charge(3)?;

    Ok(())
}

pub fn execute_stack_pop(_op: Opcode, ctx: &mut ExecutionContext) -> Result<(), Error> {
    let _ = &ctx.machine.stack.pop();
    Ok(())
}

pub fn execute_stack_duplicate(
    _op: Opcode,
    index: usize,
    ctx: &mut ExecutionContext,
) -> Result<(), Error> {
    let stack = &mut ctx.machine.stack;

    let contents = stack.get_contents();

    let dup_index = contents.iter().enumerate().len() - index;
    println!("{}", dup_index);

    if dup_index > stack.get_contents().len() {
        return Err(Error::StackError(StackError::InvalidItem));
    }

    let value = contents.get(dup_index).ok_or(StackError::EmptyStack)?;

    stack.push(*value)?;

    ctx.machine.pc += index;

    ctx.gas_meter.charge(3)?;

    Ok(())
}

pub fn execute_stack_swap(
    op: Opcode,
    position: usize,
    ctx: &mut ExecutionContext,
) -> Result<(), Error> {
    _ = &mut ctx.machine.stack.swap(position - 1)?;
    ctx.gas_meter.charge(3)?;
    Ok(())
}
