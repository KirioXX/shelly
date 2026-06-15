use shelly::{commands::skills::SkillsCommands, *};

use std::error::Error;
use std::io;

use clap::{CommandFactory, Parser};
use clap_complete::generate;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup {} => match commands::setup::setup() {
            Ok(Some(new_cfg)) => match confy::store(APP_NAME, CONFIG_NAME, new_cfg) {
                Ok(_) => println!("All done!"),
                Err(_) => eprintln!("Config save failed"),
            },
            Ok(None) => println!("Setup cancelled."),
            Err(_err) => eprintln!("Setup failed"),
        },
        Commands::Generate {
            prompt,
            dry_run,
            skills,
        } => match commands::ai::call(prompt, dry_run, skills).await {
            Ok(command) => println!("{}", command),
            Err(err) => println!("Failed: {:?}", err),
        },
        Commands::Cmds {} => {
            // Dynamically list all subcommands from the enum
            let cmd =
                <Commands as clap::Subcommand>::augment_subcommands(clap::Command::new("shelly"));
            for sub in cmd.get_subcommands() {
                println!("{}", sub.get_name());
            }
        }
        Commands::Skills(skills_cmd) => match skills_cmd {
            SkillsCommands::List {} => {
                if let Err(err) = commands::skills::list() {
                    eprintln!("Failed to list skills: {:?}", err);
                }
            }
            SkillsCommands::Add { url, skill } => {
                if let Err(err) = commands::skills::add(url, skill).await {
                    eprintln!("Failed to install skill(s): {:?}", err);
                    std::process::exit(1);
                }
            }
        },
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut io::stdout());
        }
        Commands::Config { show } => {
            if let Err(err) = commands::config::config(show) {
                eprintln!("Failed to show/edit config: {:?}", err);
                std::process::exit(1);
            }
        }
        Commands::History {
            limit,
            search,
            clear,
            raw,
        } => {
            if let Err(err) = commands::history::history(limit, search, clear, raw) {
                eprintln!("Failed to show history: {:?}", err);
                std::process::exit(1);
            }
        }
        Commands::Undo { index, dry_run } => {
            if let Err(err) = commands::undo::undo(index, dry_run) {
                eprintln!("Failed to undo: {:?}", err);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
