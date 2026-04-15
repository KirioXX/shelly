use std::error::Error;

use dialoguer::{Confirm, Input, Password, Select, console::Style, theme::ColorfulTheme};

use crate::config::{Config, Shell};

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

    let model = Input::with_theme(&theme)
        .with_prompt("AI Model")
        .default("gemma4:31b".parse().unwrap())
        .interact()?;

    let api_url = Input::with_theme(&theme)
        .with_prompt("AI Api Endpoint")
        .default("http://127.0.0.1".parse().unwrap())
        .interact()?;

    let api_key = Password::with_theme(&theme)
        .with_prompt("AI Api Key")
        .interact()?;

    let shell_selection = Select::with_theme(&theme)
        .with_prompt("Configure shell")
        .default(0)
        .item("Bash")
        .item("Zsh")
        .interact()?;

    let shell = match shell_selection {
        0 => Some(Shell::Bash),
        1 => Some(Shell::Zsh),
        _ => Some(Shell::Bash)
    };

    Ok(Some(Config {
        model,
        api_url,
        api_key,
        shell
    }))
}
