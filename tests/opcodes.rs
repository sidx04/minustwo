// use minustwo::opcodes::{Opcode, arithmetic::execute_arithmetic};
// use minustwo::{machine::Machine, opcodes::stack_ops::execute_stack_push};
// use primitive_types::U256;
// use std::rc::Rc;
// mod tests {

//     use minustwo::opcodes::{
//         memory_ops::execute_memory,
//         stack_ops::{execute_stack_duplicate, execute_stack_swap},
//     };

//     use super::*;

//     #[test]
//     fn test_push1() {
//         let code = Rc::new(vec![0x60, 0x0A]);
//         let mut machine = Machine::new(code, Rc::new(vec![]), 1024);

//         execute_stack_push(Opcode::PUSH1, 1, &mut ctx.machine).unwrap();
//         assert_eq!(ctx.machine.stack.pop().unwrap(), U256::from(10));
//         assert_eq!(ctx.machine.pc, 2);
//     }

//     #[test]
//     fn test_push8() {
//         let code = Rc::new(vec![0x67, 0xFFFFFFFFFFFFFFFF]);
//         let mut machine = Machine::new(code, Rc::new(vec![]), 1024);

//         execute_stack_push(Opcode::PUSH8, 8, &mut ctx.machine).unwrap();
//         // assert_eq!(ctx.machine.stack.pop().unwrap(), U256::from(10));
//         // assert_eq!(ctx.machine.pc, 2);
//     }

//     #[test]
//     fn test_duplicate() {
//         let code = Rc::new(vec![0x01, 0x00, 0x00]);
//         let mut machine = Machine::new(code.clone(), Rc::new(vec![]), 1024);

//         let a = code[0];
//         let b = code[1];
//         let c = code[2];

//         ctx.machine.stack.push(U256::from(a)).unwrap();
//         ctx.machine.stack.push(U256::from(b)).unwrap();
//         ctx.machine.stack.push(U256::from(c)).unwrap();

//         println!("Before duplicating:\n{}", ctx.machine.stack);

//         execute_stack_duplicate(Opcode::DUP3, 3, &mut ctx.machine).unwrap();

//         println!("After duplicating:\n{}", ctx.machine.stack);
//     }

//     #[test]
//     #[should_panic]
//     fn test_duplicate_panic() {
//         let code = Rc::new(vec![0x01, 0x00, 0x00]);
//         let mut machine = Machine::new(code.clone(), Rc::new(vec![]), 1024);

//         let a = code[0];
//         let b = code[1];

//         ctx.machine.stack.push(U256::from(a)).unwrap();
//         ctx.machine.stack.push(U256::from(b)).unwrap();

//         println!("Before duplicating:\n{}", ctx.machine.stack);

//         execute_stack_duplicate(Opcode::DUP3, 3, &mut ctx.machine).unwrap();

//         println!("After duplicating:\n{}", ctx.machine.stack);
//     }

//     #[test]
//     fn test_add() {
//         let code = Rc::new(vec![0x10, 0x0A]);
//         let mut machine = Machine::new(code.clone(), Rc::new(vec![]), 1024);
//         let a = code[0];
//         let b = code[1];
//         ctx.machine.stack.push(U256::from(a)).unwrap();
//         ctx.machine.stack.push(U256::from(b)).unwrap();

//         execute_arithmetic(Opcode::ADD, &mut ctx.machine).unwrap();

//         assert_eq!(ctx.machine.stack.pop().unwrap(), U256::from(0x1A));
//     }

//     #[test]
//     fn get_memory_length() {
//         let mut machine = Machine::new(Rc::new(vec![]), Rc::new(vec![]), 1024);
//         let value = U256::from_str_radix("0x9361005EA8041821edF4BeaF5B0518d9e75AeB13", 16).unwrap();
//         let be_val: [u8; 32] = value.to_big_endian();

//         let start_index = be_val.iter().position(|&val| val != 0).unwrap_or(32);
//         let res = &be_val[start_index..];

//         ctx.machine.memory.store(0, &res).unwrap();

//         execute_memory(Opcode::MSIZE, &mut ctx.machine).unwrap();

//         println!("{}", ctx.machine.memory);

//         assert_eq!(ctx.machine.stack.pop().unwrap(), U256::from(20));
//     }

//     #[test]
//     fn test_swap() {
//         let code = Rc::new(vec![0x01, 0x02, 0x03, 0x04, 0x06]);
//         let mut machine = Machine::new(code.clone(), Rc::new(vec![]), 1024);

//         ctx.machine.stack.push(U256::from(code[0])).unwrap();
//         ctx.machine.stack.push(U256::from(code[1])).unwrap();
//         ctx.machine.stack.push(U256::from(code[2])).unwrap();
//         ctx.machine.stack.push(U256::from(code[3])).unwrap();
//         ctx.machine.stack.push(U256::from(code[4])).unwrap();

//         println!("Before swap:\n{}", ctx.machine.stack);

//         execute_stack_swap(Opcode::SWAP3, 3, &mut ctx.machine).unwrap();

//         println!("After swap:\n{}", ctx.machine.stack);
//     }
// }
