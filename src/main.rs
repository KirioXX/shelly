mod config;
mod commands;
mod skills;
mod tools;

use std::error::Error;
use std::io;

use clap::{Parser, Subcommand, CommandFactory};
use clap_complete::{generate, shells};

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
    /// Run the setup wizard
    Setup {},
    /// Generate shell commands from natural language
    Generate {
        #[arg(trailing_var_arg = true)]
        prompt: Vec<String>,

        #[arg(long, help = "Show command without executing")]
        dry_run: bool,
    },
    /// List all available subcommands
    Cmds {},
    /// Generate shell completion scripts
    Completions {
        #[arg(value_enum)]
        shell: shells::Shell,
    },}

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
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut io::stdout());
        }
    }
    Ok(())
}
