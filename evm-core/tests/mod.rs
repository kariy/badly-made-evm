#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use ethereum_types::U256;
    use evm_core::{evm::GlobalEnvironment, executor::ExecutionContext};

    #[test]
    fn simple_arithmetic_operations() {
        let program = vec![
            0x60, 0x03, 0x60, 0x03, 0x01, 0x60, 0x03, 0x01, 0x60, 0x1B, 0x04, 0x60, 0x03, 0x02,
        ];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());
        assert_eq!(context.execution_machine.stack.height(), 1);
        assert_eq!(
            U256::from(0x09),
            context.execution_machine.stack.pop().unwrap()
        );
    }

    #[test]
    fn swap_operations() {
        let program = vec![0x60, 0x69, 0x60, 0x33, 0x90];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());
        assert_eq!(
            context.execution_machine.stack.get_from_top(0).unwrap(),
            U256::from(0x69)
        );
        assert_eq!(
            context.execution_machine.stack.get_from_top(1).unwrap(),
            U256::from(0x33)
        );
    }

    #[test]
    fn dup_operations() {
        let program = vec![0x60, 0x69, 0x60, 0x33, 0x80];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());
        assert_eq!(context.execution_machine.stack.height(), 3);
        assert_eq!(
            context.execution_machine.stack.get_from_top(0).unwrap(),
            U256::from(0x33)
        );
        assert_eq!(
            context.execution_machine.stack.get_from_top(1).unwrap(),
            U256::from(0x33)
        );
    }

    #[test]
    fn push_operations() {
        let program = vec![
            0x62, 0x42, 0x00, 0x69, 0x60, 0x33, 0x61, 0x00, 0x23, 0x60, 0x99,
        ];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());
        assert_eq!(context.execution_machine.stack.height(), 4);
        assert_eq!(
            context.execution_machine.stack.get_from_top(0).unwrap(),
            U256::from(0x99)
        );
        assert_eq!(
            context.execution_machine.stack.get_from_top(1).unwrap(),
            U256::from(0x0023)
        );
        assert_eq!(
            context.execution_machine.stack.get_from_top(3).unwrap(),
            U256::from(0x420069)
        );
    }

    #[test]
    fn memory_operations() {
        let program = vec![
            0x62, 0x00, 0x23, 0x44, 0x60, 0x00, 0x52, 0x60, 0x00, 0x51, 0x60, 0x00, 0x51,
        ];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());

        let value = context.execution_machine.memory.read_bytes(0, 32);

        assert!(context.execution_machine.stack.height() == 2);
        assert!(32 == context.execution_machine.memory.used_capacity());
        assert_eq!(U256::from_big_endian(&value), U256::from(0x002344));
        assert_eq!(
            U256::from(0x002344),
            context.execution_machine.stack.get_from_top(0).unwrap()
        );
        assert_eq!(
            U256::from(0x002344),
            context.execution_machine.stack.get_from_top(1).unwrap()
        );
    }
}
