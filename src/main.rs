mod config;
mod commands;

use clap::{Parser, Subcommand};

const APP_NAME: &str = "shelly";
const CONFIG_NAME: &str = "config";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    Setup {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
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
    }
}
