use shelly::*;

use std::error::Error;
use std::io;

use clap::{Parser, Subcommand, CommandFactory};
use clap_complete::{generate, shells};

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
    /// Manage skills
    #[command(subcommand)]
    Skills(SkillsCommands),
    /// Generate shell completion scripts
    Completions {
        #[arg(value_enum)]
        shell: shells::Shell,
    },
}

#[derive(Debug, Subcommand)]
enum SkillsCommands {
    /// List installed skills
    List {},
    /// Install a skill from a GitHub repository
    Add {
        /// GitHub URL or user/repo shorthand
        url: String,
    },
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
        Commands::Skills(skills_cmd) => {
            match skills_cmd {
                SkillsCommands::List {} => {
                    if let Err(err) = commands::skills::list() {
                        eprintln!("Failed to list skills: {:?}", err);
                    }
                }
                SkillsCommands::Add { url } => {
                    if let Err(err) = commands::skills::add(url).await {
                        eprintln!("Failed to install skill: {:?}", err);
                        std::process::exit(1);
                    }
                }
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
