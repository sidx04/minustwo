use std::rc::Rc;

use primitive_types::H160;

use crate::{errors::Error, execution::ExecutionContext, machine::Machine, utils::hash_keccak};

use super::Opcode;

pub fn execute_create(op: Opcode, ctx: &mut ExecutionContext) -> Result<(), Error> {
    let stack = &mut ctx.machine.stack;
    let memory = &mut ctx.machine.memory;
    let fund_value = stack.pop()?; // value in wei to send to the new account
    let offset = stack.pop()?; // byte offset in the memory in bytes, the initialisation code for the new account
    let size = stack.pop()?; // byte size to copy (size of the initialisation code)

    let mut code = memory
        .access(offset.as_usize(), size.as_usize())?
        .iter()
        .map(|x| (*x).into())
        .collect::<Vec<usize>>();

    // replace with `generate_address()`;
    let addr = H160::from_low_u64_be(0xAABB);

    ctx.state.create_account(addr, fund_value, code.clone());

    let code_rc = Rc::new(code.iter().map(|b| *b as usize).collect::<Vec<_>>());

    let machine = Box::leak(Box::new(Machine::new(code_rc, Rc::new(vec![]), 1024)));

    // 5. Create a sub-ctx
    // let mut sub_ctx = ExecutionContext {
    //    machine,
    //    state: ctx.state,
    //    caller: ctx.callee,
    //    callee: addr,
    //    value: fund_value,
    //    gas_meter: ctx.gas_meter, // or a new one with reduced gas
    // };

    // 6. Execute the init code
    // sub_ctx.run(); // implement `execute` if needed

    // 7. On success, set the code of the new contract to return data from init execution
    // let return_data = sub_ctx.machine.memory.dump_return_data(); // you define this method
    // ctx.state.set_code(new_address, return_data); // You implement this

    // 8. Push address to stack
    // ctx.machine
    //    .stack
    //    .push(U256::from_big_endian(&new_address.0))?;
    Ok(())
}
