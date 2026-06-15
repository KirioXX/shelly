pub mod ai;
pub mod config;
pub mod history;
pub mod setup;
pub mod skills;
pub mod undo;

use clap::Subcommand;
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
    /// View or edit configuration
    Config {
        /// Display config without editing
        #[arg(short, long)]
        show: bool,
    },
    /// Browse command history
    History {
        #[arg(short, long, default_value = "20")]
        limit: usize,
        #[arg(short, long)]
        search: Option<String>,
        #[arg(long)]
        clear: bool,
        #[arg(long)]
        raw: bool,
    },
    /// Replay a previous command
    Undo {
        #[arg(short, long, default_value = "0")]
        index: usize,
        #[arg(short = 'd', long)]
        dry_run: bool,
    },
}
