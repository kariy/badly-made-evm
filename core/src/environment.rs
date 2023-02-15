use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use color_eyre::owo_colors::OwoColorize;
use ethereum_types::{H160, H256, U256};

#[derive(Debug, Default)]
pub struct AccountState {
    pub balance: U256,
    pub code: Option<Vec<u8>>,
    pub storage: BTreeMap<U256, U256>,
}

pub type GlobalStorage = BTreeMap<H160, AccountState>;

#[derive(Debug, Default, Clone)]
pub struct CurrentBlockInformation {
    pub timestamp: u128,
    pub gas_limit: u128,
    pub block_hash: H256,
    pub block_number: u128,
}

#[derive(Debug, Default)]
pub struct GlobalEnvironment {
    pub chain_id: u32,
    pub current_block: CurrentBlockInformation,
    pub global_storage: Rc<RefCell<GlobalStorage>>,
}

pub struct ExecutionResult {
    pub data: Vec<u8>,
}

impl std::fmt::Display for ExecutionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"{}
0x{}
",
            "[ Output ]".purple(),
            self.data
                .iter()
                .map(|i| format!("{i:02x}"))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
