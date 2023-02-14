use std::{cell::RefCell, rc::Rc};

use crate::environment::{CurrentBlockInformation, GlobalEnvironment, GlobalStorage};
use crate::executor::{ExecutionContext, ExecutionEnvironment};
use color_eyre::Result;

#[derive(Debug, Default, Clone)]
pub struct EvmConfig {
    chain_id: u32,
    current_block: CurrentBlockInformation,
    root_execution_env: ExecutionEnvironment,
}

#[derive(Default)]
pub struct Evm {
    config: EvmConfig,
    global_env: Rc<GlobalEnvironment>,
}

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

    /// Will execute based on the provided program
    pub fn boring_run(&mut self, program: Vec<u8>) -> Result<Vec<u8>> {
        let mut executor = ExecutionContext::new(
            self.config.root_execution_env.clone(),
            self.global_env.clone(),
        );
        let result = executor.run(program)?;
        Ok(result)
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

        let result = evm.boring_run(program).unwrap();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn not_so_simple_boring_run() {
        let program = vec![
            0x60, 0x69, 0x80, 0x14, 0x15, 0x61, 0x20, 0x77, 0x60, 0x00, 0x52, 0x60, 0x02, 0x60,
            0x00, 0xF3,
        ];
        let mut evm = Evm::new_with_config(EvmConfig {
            ..Default::default()
        });

        let result = evm.boring_run(program).unwrap();
        assert_eq!(result, vec![0x20, 0x77]);
    }
}
