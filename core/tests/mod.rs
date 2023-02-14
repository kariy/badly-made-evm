#[cfg(test)]
mod tests {
    use ethereum_types::U256;
    use evm_core::executor::ExecutionContext;

    #[test]
    fn simple_arithmetic_operations() {
        let program = vec![
            0x60, 0x03, 0x60, 0x03, 0x01, 0x60, 0x03, 0x01, 0x60, 0x1B, 0x04, 0x60, 0x03, 0x02,
        ];
        let mut context = ExecutionContext::default();

        assert!(context.run(program).is_ok());
        assert_eq!(context.execution_machine.stack.height(), 1);
        assert_eq!(
            U256::from(0x09),
            context.execution_machine.stack.pop().unwrap()
        );
    }

    #[test]
    fn comparison_operations() {
        let program = vec![0x60, 0x01, 0x60, 0x20, 0x10, 0x15, 0x80, 0x14, 0x15, 0x15];
        let mut context = ExecutionContext::default();

        assert!(context.run(program).is_ok());
        assert_eq!(U256::one(), context.execution_machine.stack.pop().unwrap());
    }

    #[test]
    fn bitwise_operations() {
        let program = vec![
            0x60, 0x01, 0x60, 0x02, 0x16, 0x60, 0x01, 0x17, 0x60, 0x03, 0x1B, 0x60, 0x01, 0x17,
            0x60, 0b1101, 0x18,
        ];
        let mut context = ExecutionContext::default();

        assert!(context.run(program).is_ok());
        assert_eq!(
            U256::from(0b0100),
            context.execution_machine.stack.pop().unwrap()
        );
    }

    #[test]
    fn swap_operations() {
        let program = vec![0x60, 0x69, 0x60, 0x33, 0x90];
        let mut context = ExecutionContext::default();

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
        let mut context = ExecutionContext::default();

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
        let mut context = ExecutionContext::default();

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
        let mut context = ExecutionContext::default();

        assert!(context.run(program).is_ok());

        let expected_value = U256::from_str_radix(
            "0023440000000000000000000000000000000000000000000000000000000000",
            16,
        )
        .unwrap();

        let value = context.execution_machine.memory.read_bytes(0, 3);

        assert!(context.execution_machine.stack.height() == 2);
        assert!(
            32 == context.execution_machine.memory.used_capacity(),
            "memory expanded more than expected"
        );

        assert_eq!(U256::from_big_endian(&value), U256::from(0x002344));
        assert_eq!(
            expected_value,
            context.execution_machine.stack.get_from_top(0).unwrap()
        );
        assert_eq!(
            expected_value,
            context.execution_machine.stack.get_from_top(1).unwrap()
        );
    }

    #[test]
    fn jump_operations() {
        let program = vec![
            0x60, 0x69, 0x80, 0x14, 0x60, 0x09, 0x57, 0xF3, 0xF3, 0x5B, 0x60, 0x00, 0x60, 0x00,
            0x57, 0x58, 0x60, 15, 0x14, 0x00,
        ];
        let mut context = ExecutionContext::default();

        assert!(context.run(program).is_ok());
        assert_eq!(
            context.execution_machine.stack.pop().unwrap(),
            U256::from(1)
        );
    }

    #[test]
    fn jump_must_fail_if_not_jumpdest() {
        let program = vec![
            0x60, 0x69, 0x80, 0x14, 0x60, 0x09, 0x57, 0xF3, 0xF3, 0x60, 0x01,
        ];
        let mut context = ExecutionContext::default();

        assert!(context.run(program).is_err());
    }

    #[test]
    fn test_sha3_precompiled() {
        let program = vec![
            0x62, 0x69, 0x12, 0x99, 0x60, 0x00, 0x52, 0x60, 0x03, 0x60, 0x00, 0x20, 0x61, 0x69,
            0x12, 0x60, 0x00, 0x52, 0x60, 0x02, 0x60, 0x00, 0x20,
        ];
        let mut context = ExecutionContext::default();

        assert!(context.run(program).is_ok());
        assert_eq!(
            context.execution_machine.stack.pop().unwrap(),
            U256::from_str_radix(
                "792a606aa2618dfd7783da32b4584fbf99033bf61c3b91c766aa08fd7dd2a4b4",
                16
            )
            .unwrap()
        );
        assert_eq!(
            context.execution_machine.stack.pop().unwrap(),
            U256::from_str_radix(
                "a63d3141e51e0d7dd85f429012c8b62d573940ff51a7378f3739522a038a0d99",
                16
            )
            .unwrap()
        )
    }
}
