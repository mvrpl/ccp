mod messengers;
mod utils;

use clap::{Args, Parser, Subcommand};
use clap_stdin::MaybeStdin;
use lazy_static::lazy_static;
use securestore::SecretsManager;
use tempdir::TempDir;

use messengers::sender::Sender;
use utils::vault::Vault;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
struct CmdCpArgs {
    input_file: MaybeStdin<String>,
    #[arg(long, short)]
    file_name: Option<String>,
    messenger_target: String,
}

struct CpArgs {
    input_file: std::path::PathBuf,
    messenger_target: String,
}

#[derive(Args, Debug)]
struct CmdMvArgs {
    input_file: std::path::PathBuf,
    messenger_target: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Cp(CmdCpArgs),
    Mv(CmdMvArgs),
}

lazy_static! {
    static ref VAULT: SecretsManager = Vault::init();
    static ref TEMP_DIR: TempDir = TempDir::new("ccp").expect("Failed create temporary dir");
}

pub fn drop_temp() {
    std::fs::remove_dir_all(TEMP_DIR.path()).expect("Failed remove temporary dir")
}

fn main() {
    let result = std::panic::catch_unwind(|| {
        let args = Cli::parse();
        match args.command {
            Commands::Cp(cmd_args) => {
                let in_file = match std::fs::metadata(cmd_args.input_file.clone().into_inner()) {
                    Ok(file) if file.is_file() => {
                        std::path::PathBuf::from(cmd_args.input_file.clone().into_inner())
                    }
                    Ok(_) => panic!("Input_file is not a file"),
                    Err(_) => {
                        let temp_file_path = TEMP_DIR
                            .path()
                            .join(cmd_args.file_name.expect("Send -f or --file-name"));
                        std::fs::write(&temp_file_path, cmd_args.input_file.into_inner()).ok();
                        temp_file_path
                    }
                };
                let args = CpArgs {
                    input_file: in_file,
                    messenger_target: cmd_args.messenger_target,
                };
                Sender::copy_file_to_chat(args)
            }
            Commands::Mv(cmd_args) => {
                assert!(std::fs::metadata(cmd_args.input_file.clone().into_os_string().into_string().unwrap()).unwrap().is_file(), "Input_file is not a file");
                let args = CpArgs {
                    input_file: cmd_args.input_file.clone(),
                    messenger_target: cmd_args.messenger_target,
                };
                Sender::copy_file_to_chat(args);
                std::fs::remove_file(cmd_args.input_file).expect("Failed on remove input_file")
            }
        }
    });
    match result {
        Ok(_) => drop_temp(),
        Err(_) => {
            drop_temp();
            std::process::exit(1)
        }
    }
}
