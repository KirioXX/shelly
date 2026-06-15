use crate::skills::SkillManager;
use clap::Subcommand;
use console::style;
use std::error::Error;

#[derive(Debug, Subcommand)]
pub enum SkillsCommands {
    /// List installed skills
    List {},
    /// Install skills from a GitHub repository
    Add {
        /// GitHub URL or user/repo shorthand (e.g., owner/repo)
        url: String,
        /// Specific skill name to install (optional - installs all if not specified)
        #[arg(long)]
        skill: Option<String>,
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

/// Install skill(s) from a GitHub URL
pub async fn add(url: String, specific_skill: Option<String>) -> Result<(), Box<dyn Error>> {
    let manager = SkillManager::new()?;

    eprintln!(
        "{}",
        style(format!("Attempting to install from: {}", url)).dim()
    );

    match manager
        .install_from_url(&url, specific_skill.as_deref())
        .await
    {
        Ok(installed) => {
            if installed.is_empty() {
                println!("\n{}", style("No skills were installed.").yellow());
            } else if installed.len() == 1 {
                println!(
                    "\n{}",
                    style(format!("✓ Successfully installed '{}'", installed[0]))
                        .green()
                        .bold()
                );
            } else {
                println!(
                    "\n{}",
                    style(format!(
                        "✓ Successfully installed {} skills:",
                        installed.len()
                    ))
                    .green()
                    .bold()
                );
                for name in &installed {
                    println!("    • {}", style(name).cyan());
                }
            }
            println!("\nUse 'shelly skills list' to see all installed skills.");
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
