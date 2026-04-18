use std::error::Error;
use std::fs::{OpenOptions};
use std::io::{Write};

use dialoguer::{Confirm, Input, Password, Select, console::Style, theme::ColorfulTheme};

use crate::config::{Config, Shell};

const BASH_WRAPPER: &str = include_str!("scripts/bash.sh");
const ZSH_WRAPPER: &str = include_str!("scripts/zsh.sh");
const FISH_WRAPPER: &str = include_str!("scripts/fish.sh");

// Init the tool in the users system
pub fn setup() -> Result<Option<Config>, Box<dyn Error>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("Welcome to the setup wizard");

    if !Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
        .interact()?
    {
        return Ok(None);
    }

    let model: String = Input::with_theme(&theme)
        .with_prompt("AI Model")
        .default("gemma4:31b-cloud".parse().unwrap())
        .interact()?;

    let api_url: String = Input::with_theme(&theme)
        .with_prompt("AI Api Endpoint")
        .default("https://ollama.com/v1".parse().unwrap())
        .interact()?;

    let api_key = Password::with_theme(&theme)
        .with_prompt("AI Api Key")
        .interact()?;

    let shell_selection = Select::with_theme(&theme)
        .with_prompt("Configure shell")
        .default(0)
        .item("Bash")
        .item("Zsh")
        .item("Fish")
        .interact()?;

    let shell = match shell_selection {
        0 => Shell::Bash,
        1 => Shell::Zsh,
        2 => Shell::Fish,
        _ => Shell::Bash,
    };

    // Shell Integration
    let (config_file, wrapper) = match shell {
        Shell::Bash => {
            let mut path = dirs::home_dir().ok_or("Could not find home directory")?;
            path.push(".bashrc");
            (path, BASH_WRAPPER)
        }
        Shell::Zsh => {
            let mut path = dirs::home_dir().ok_or("Could not find home directory")?;
            path.push(".zshrc");
            (path, ZSH_WRAPPER)
        }
        Shell::Fish => {
            let mut path = dirs::home_dir().ok_or("Could not find home directory")?;
            path.push(".config/fish/config.fish");
            (path, FISH_WRAPPER)
        }
    };

    println!("\nProposed shell wrapper:\n{}", wrapper);

    if Confirm::with_theme(&theme)
        .with_prompt(&format!("Would you like to add this to your {:?}?", config_file))
        .interact()?
    {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&config_file)?;

        writeln!(file, "# shelly\n{}\n# shelly end\n", wrapper)?;
        println!("Shell integration added successfully!");
        println!("")
    }

    Ok(Some(Config {
        model,
        api_url,
        api_key,
        shell: Some(shell),
    }))
}
