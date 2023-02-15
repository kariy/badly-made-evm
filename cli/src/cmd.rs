use clap::{Parser, Subcommand};
use color_eyre::Result;
use ethereum_types::{H160, U256};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct App {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(name = "exec-boring")]
    #[clap(visible_alias = "xb")]
    ExecuteBoring {
        #[clap(required(true))]
        bytecode: String,

        #[clap(short, long)]
        #[clap(default_value = "0")]
        value: U256,

        #[clap(short, long)]
        #[clap(default_value = "0x0000000000000000000000000000000000000000")]
        caller: H160,

        #[clap(short = 'd', long)]
        #[clap(default_value = "0x00")]
        calldata: String,

        #[clap(short = 'C', long)]
        #[clap(default_value = "0x0000000000000000000000000000000000000000")]
        contract_address: H160,
    },
}

pub fn decode_hex_string(hex: &str) -> Result<Vec<u8>> {
    if let Some(hex_str) = hex.strip_prefix("0x") {
        Ok(hex::decode(hex_str)?)
    } else {
        Ok(hex::decode(hex)?)
    }
}
