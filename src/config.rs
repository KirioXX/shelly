use clap::ValueEnum;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shell::Bash => write!(f, "Bash"),
            Shell::Zsh => write!(f, "Zsh"),
            Shell::Fish => write!(f, "Fish"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub model: String,
    pub api_url: String,
    pub api_key: String,
    pub shell: Option<Shell>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            model: "".into(),
            api_url: "".into(),
            api_key: "".into(),
            shell: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_display() {
        assert_eq!(Shell::Bash.to_string(), "Bash");
        assert_eq!(Shell::Zsh.to_string(), "Zsh");
        assert_eq!(Shell::Fish.to_string(), "Fish");
    }

    #[test]
    fn test_shell_value_enum() {
        // Shell implements ValueEnum for clap
        let shells = Shell::value_variants();
        assert_eq!(shells.len(), 3);
    }
}
