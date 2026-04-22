use crate::skills::SkillManager;
use console::style;
use std::error::Error;
use clap::{Subcommand};

#[derive(Debug, Subcommand)]
pub enum SkillsCommands {
    /// List installed skills
    List {},
    /// Install a skill from a GitHub repository
    Add {
        /// GitHub URL or user/repo shorthand
        url: String,
    },
}

/// List all installed skills
pub fn list() -> Result<(), Box<dyn Error>> {
    let manager = SkillManager::new()?;
    let skills = manager.discover_skills()?;

    if skills.is_empty() {
        println!("{}", style("No skills installed.").dim());
        println!("\nSkills are loaded from: ~/.config/shelly/skills/");
        println!("Each skill should be a directory containing a SKILL.md file.");
    } else {
        println!(
            "{}",
            style(format!("Installed skills ({}):", skills.len())).bold()
        );
        println!();

        for skill in skills {
            println!(
                "  {} - {}",
                style(&skill.name).cyan().bold(),
                skill.description
            );
        }
    }

    Ok(())
}

/// Install a skill from a GitHub URL
pub async fn add(url: String) -> Result<(), Box<dyn Error>> {
    let manager = SkillManager::new()?;

    eprintln!(
        "{}",
        style(format!("Attempting to install skill from: {}", url)).dim()
    );

    match manager.install_from_url(&url).await {
        Ok(skill_name) => {
            println!(
                "\n{}",
                style(format!("✓ Successfully installed '{}'", skill_name))
                    .green()
                    .bold()
            );
            println!("Use 'shelly skills list' to see all installed skills.");
        }
        Err(e) => {
            eprintln!(
                "\n{}",
                style(format!("✗ Installation failed: {}", e)).red().bold()
            );
            std::process::exit(1);
        }
    }

    Ok(())
}
