pub mod config;
pub mod commands;
pub mod history;
pub mod skills;
pub mod tools;

pub const APP_NAME: &str = "shelly";
pub const CONFIG_NAME: &str = "config";

pub use config::*;
pub use commands::*;
pub use skills::*;
pub use tools::*;
