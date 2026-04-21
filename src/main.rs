mod config;
mod commands;
mod skills;

use std::error::Error;

use clap::{Parser, Subcommand};

pub const APP_NAME: &str = "shelly";
pub const CONFIG_NAME: &str = "config";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Setup {},
    Generate {
        #[arg(trailing_var_arg = true)]
        prompt: Vec<String>,

        #[arg(long, help = "Show command without executing")]
        dry_run: bool,
    },
    /// List all available subcommands
    Cmds {},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup {} => {
            match commands::setup::setup() {
                Ok(Some(new_cfg)) => {
                    match confy::store(
                        APP_NAME,
                        CONFIG_NAME,
                        new_cfg
                    ) {
                        Ok(_) => println!("All done!"),
                        Err(_) => eprintln!("Config save failed")
                    }
                }
                Ok(None) => println!("Setup cancelled."),
            Err(_err) => eprintln!("Setup failed")
            }
        }
        Commands::Generate { prompt, dry_run } => {
            match commands::ai::call(prompt, dry_run).await {
                Ok(command) => println!("{}", command),
                Err(err) => println!("Failed: {:?}", err)
            }
        }
        Commands::Cmds {} => {
            // Dynamically list all subcommands from the enum
            let cmd = <Commands as clap::Subcommand>::augment_subcommands(clap::Command::new("shelly"));
            for sub in cmd.get_subcommands() {
                println!("{}", sub.get_name());
            }
        }
    }
    Ok(())
}
