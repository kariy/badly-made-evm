use std::{cell::RefCell, rc::Rc};

use crate::environment::{CurrentBlockInformation, GlobalEnvironment, GlobalStorage};
use crate::executor::{ExecutionContext, ExecutionEnvironment};

#[derive(Debug, Default, Clone)]
pub struct EvmConfig {
    pub chain_id: u32,
    pub current_block: CurrentBlockInformation,
    pub root_execution_env: ExecutionEnvironment,
}

#[derive(Default)]
pub struct Evm {
    config: EvmConfig,
    global_env: Rc<GlobalEnvironment>,
}

// TODO: its ugly and doesnt make sense pls fix
impl Evm {
    pub fn new_with_config(config: EvmConfig) -> Self {
        Self {
            config: config.clone(),
            global_env: Rc::new(GlobalEnvironment {
                chain_id: config.chain_id,
                current_block: config.current_block,
                global_storage: Rc::new(RefCell::new(GlobalStorage::default())),
            }),
        }
    }

    pub fn build_executor(&mut self) -> ExecutionContext {
        ExecutionContext::new(
            self.config.root_execution_env.clone(),
            self.global_env.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_boring_run() {
        let program = vec![0x60, 0x69, 0x80, 0x14, 0x15];
        let mut evm = Evm::new_with_config(EvmConfig {
            ..Default::default()
        });

        let mut exec = evm.build_executor();
        let result = exec.run(program).unwrap();

        assert_eq!(result.data, vec![]);
    }

    #[test]
    fn not_so_simple_boring_run() {
        let program = vec![
            0x60, 0x69, 0x80, 0x14, 0x15, 0x61, 0x20, 0x77, 0x60, 0x00, 0x52, 0x60, 32, 0x60, 0x00,
            0xF3,
        ];
        let mut evm = Evm::new_with_config(EvmConfig {
            ..Default::default()
        });

        let mut exec = evm.build_executor();
        let result = exec.run(program).unwrap();

        let mut expected_value = vec![0u8; 32];
        expected_value[30] = 32;
        expected_value[31] = 119;

        assert_eq!(result.data.len(), 32);
        assert_eq!(result.data, expected_value);
    }
}
