mod utils;
mod messengers;

use clap::{Args, Parser, Subcommand};
use lazy_static::lazy_static;
use securestore::SecretsManager;

use utils::vault::Vault;
use messengers::sender::Sender;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
struct CmdCpArgs {
    input_file: String,
    messenger_target: String,
}

#[derive(Args, Debug)]
struct CmdMvArgs {
    input_file: String,
    messenger_target: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Cp(CmdCpArgs),
    Mv(CmdMvArgs),
}

lazy_static! {
    static ref VAULT: SecretsManager = Vault::init();
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::Cp(cmd_args) => {
            Sender::copy_file_to_chat(cmd_args)
        }
        Commands::Mv(cmd_args) => {
            println!("{:?}", cmd_args)
        }
    }
}
