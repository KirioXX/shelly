use serde_derive::{Serialize, Deserialize};
use clap::{ValueEnum};

#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum)]
pub enum Shell {
  Bash,
  Zsh
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub model: String,
    pub api_url: String,
    pub api_key: String,
    pub shell: Option<Shell>,
}

impl ::std::default::Default for Config {
    fn default() -> Self { Self { model: "".into(), api_url: "".into(), api_key: "".into(), shell: None } }
}
