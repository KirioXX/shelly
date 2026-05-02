pub mod ai;
pub mod setup;
pub mod skills;

use clap::{Subcommand};
use clap_complete::shells;

use crate::commands::skills::SkillsCommands;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run the setup wizard
    Setup {},
    /// Generate shell commands from natural language
    Generate {
        #[arg(trailing_var_arg = true)]
        prompt: Vec<String>,

        #[arg(short = 'd', long, help = "Show command without executing")]
        dry_run: bool,
        
        #[arg(long, help = "Comma-separated list of skills to use")]
        skills: Option<String>,
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
