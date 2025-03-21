use minustwo::machine::stack::Stack;
use primitive_types::U256;

#[cfg(test)]
mod tests {

    use minustwo::{constants::MAX_STACK_DEPTH, machine::errors::StackError};

    use super::*;

    #[test]
    fn test_stack_push_pop_peek() {
        let mut stack = Stack::init();

        assert!(stack.is_empty());
        assert!(matches!(stack.peek(), Err(StackError::EmptyStack)));
        assert!(matches!(stack.pop(), Err(StackError::EmptyStack)));

        assert!(stack.push(U256::from(163)).is_ok());
        assert!(stack.push(U256::from(205)).is_ok());

        assert!(!stack.is_empty());

        let top = stack.peek().unwrap();
        assert_eq!(*top, U256::from(205));

        let popped = stack.pop().unwrap();
        assert_eq!(popped, U256::from(205));

        let popped = stack.pop().unwrap();
        assert_eq!(popped, U256::from(163));

        assert!(stack.is_empty());
        assert!(matches!(stack.pop(), Err(StackError::EmptyStack)));
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::init();

        for i in 0..MAX_STACK_DEPTH {
            assert!(stack.push(U256::from(i as u128)).is_ok());
        }

        let result = stack.push(U256::from(2));

        assert!(matches!(result, Err(StackError::StackOverflow)));
    }

    #[test]
    fn test_invalid_item() {
        let mut stack = Stack::init();

        let result = stack.push(U256::max_value());
        assert!(matches!(result, Err(StackError::InvalidItem)));
    }

    #[test]
    fn test_stack_clear() {
        let mut stack = Stack::init();

        assert!(stack.push(U256::from(5)).is_ok());
        assert!(stack.push(U256::from(15)).is_ok());
        assert!(!stack.is_empty());

        stack.clear();

        assert!(stack.is_empty());
        assert!(matches!(stack.peek(), Err(StackError::EmptyStack)));
    }
}
