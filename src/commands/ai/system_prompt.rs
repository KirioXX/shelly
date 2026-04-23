use std::{collections::BTreeMap, error::Error};

use console::style;
use handlebars::Handlebars;

use crate::{config::Config, skills::SkillManager};

const SYSTEM_PROMPT_TEMPLATE: &str = include_str!("prompts/system-prompt.md");

fn get_matching_skills(
    full_prompt: &str,
    manual_skills: &Option<String>,
) -> Result<Vec<crate::skills::Skill>, Box<dyn Error>> {
    let skill_manager = SkillManager::new()?;

    // If manual skills specified, use those
    if let Some(skill_names) = manual_skills {
        let names: Vec<String> = skill_names
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        Ok(skill_manager.load_skills_by_name(&names)?)
    } else {
        // Otherwise auto-detect matching skills
        Ok(skill_manager.find_matching_skills(full_prompt)?)
    }
}

pub fn get_system_prompt(
    full_prompt: &str,
    cfg: &Config,
    manual_skills: &Option<String>,
) -> Result<String, Box<dyn Error>> {
    // Check for matching skills
    let matching_skills = get_matching_skills(full_prompt, manual_skills)?;

    let skill_instructions: Vec<(String, String)> = matching_skills
        .iter()
        .map(|s| {
            if manual_skills.is_some() {
                eprintln!("{}", style(format!("📚 Using skill: {}", s.name)).cyan());
            } else {
                eprintln!(
                    "{}",
                    style(format!("📚 Auto-detected skill: {}", s.name))
                        .cyan()
                        .dim()
                );
            }
            (s.name.clone(), s.content.clone())
        })
        .collect();

    // Build system prompt with OS and shell info
    let handlebars = Handlebars::new();
    let mut data = BTreeMap::new();
    data.insert("os", std::env::consts::OS);
    let shell_str = cfg
        .shell
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    data.insert("shell", shell_str.as_str());

    let mut system_prompt = handlebars.render_template(SYSTEM_PROMPT_TEMPLATE, &data)?;

    // Append skill instructions if found
    if !skill_instructions.is_empty() {
        system_prompt.push_str("\n\n# Skills Context\n\n");
        system_prompt
            .push_str("The following skills have been loaded to help with this request:\n\n");

        for (name, content) in skill_instructions {
            system_prompt.push_str(&format!("## Skill: {}\n\n", name));
            system_prompt.push_str(&content);
            system_prompt.push_str("\n\n---\n\n");
        }
    }

    Ok(system_prompt)
}
