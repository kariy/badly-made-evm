mod cmd;

use clap::Parser;
use cmd::{decode_hex_string, App, Commands};
use color_eyre::Result;
use evm_core::{
    evm::{Evm, EvmConfig},
    executor::ExecutionEnvironment,
};

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let cli = App::parse();

    match cli.command {
        Commands::ExecuteBoring {
            value,
            caller,
            calldata,
            bytecode,
            contract_address,
        } => {
            let program = decode_hex_string(&bytecode)?;

            let config = EvmConfig {
                root_execution_env: ExecutionEnvironment {
                    value,
                    caller,
                    calldata: decode_hex_string(&calldata)?,
                    contract_address,
                },
                ..Default::default()
            };

            let mut evm = Evm::new_with_config(config);
            let mut executor = evm.build_executor();
            let result = executor.run(program)?;

            println!("\n{result}");
            print!("{}", executor.execution_machine);
        }
    }

    Ok(())
}
