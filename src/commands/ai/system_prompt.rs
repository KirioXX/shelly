use std::{collections::BTreeMap, error::Error};

use console::style;
use handlebars::Handlebars;

use crate::{config::Config, skills::SkillManager};

const SYSTEM_PROMPT_TEMPLATE: &str = include_str!("prompts/system-prompt.md");

pub fn get_system_prompt(full_prompt: &str, cfg: &Config) -> Result<String, Box<dyn Error>> {
    // Check for matching skill
    let skill_manager = SkillManager::new()?;
    let skill_instruction = if let Some(skill) = skill_manager.find_matching_skill(full_prompt)? {
        eprintln!("{}", style(format!("📚 Using skill: {}", skill.name)).cyan());
        Some(skill.content)
    } else {
        None
    };

    // Build system prompt with OS and shell info
    let handlebars = Handlebars::new();
    let mut data = BTreeMap::new();
    data.insert("os", std::env::consts::OS);
    let shell_str = cfg.shell.as_ref().map(|s| s.to_string()).unwrap_or_else(|| "unknown".to_string());
    data.insert("shell", shell_str.as_str());

    let mut system_prompt = handlebars.render_template(SYSTEM_PROMPT_TEMPLATE, &data)?;

    // Append skill instructions if found
    if let Some(instruction) = skill_instruction {
        system_prompt.push_str("\n\n# Skill Context\n\n");
        system_prompt.push_str(&instruction);
    }

    Ok(system_prompt)
}
