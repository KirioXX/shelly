use std::{collections::BTreeMap, error::Error};

use console::style;
use handlebars::Handlebars;

use crate::{config::Config, skills::Skill, skills::SkillManager};

const SYSTEM_PROMPT_TEMPLATE: &str = include_str!("prompts/system-prompt.md");

fn get_matching_skills(
    full_prompt: &str,
    manual_skills: &Option<String>,
) -> Result<Vec<Skill>, Box<dyn Error>> {
    let skill_manager = SkillManager::new()?;

    if let Some(skill_names) = manual_skills {
        let names: Vec<String> = skill_names
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        Ok(skill_manager.load_skills_by_name(&names)?)
    } else {
        Ok(skill_manager.find_matching_skills(full_prompt)?)
    }
}

fn build_system_prompt(os: &str, shell: &str, skills: &[Skill]) -> Result<String, Box<dyn Error>> {
    let handlebars = Handlebars::new();
    let mut data = BTreeMap::new();
    data.insert("os", os);
    data.insert("shell", shell);

    let mut system_prompt = handlebars.render_template(SYSTEM_PROMPT_TEMPLATE, &data)?;

    if !skills.is_empty() {
        system_prompt.push_str("\n\n# Available Skills\n\n");
        system_prompt.push_str(
            "The following skills may be relevant to your request. \
             Use the `read_file` tool to read the full instructions only when needed:\n\n",
        );

        for skill in skills {
            system_prompt.push_str(&format!("- **{}**: {}\n", skill.name, skill.description));
            system_prompt.push_str(&format!("  File: `{}`\n", skill.path.display()));
        }

        system_prompt
            .push_str("\nOnly read a skill if its description matches the current task.\n");
    }

    Ok(system_prompt)
}

pub fn get_system_prompt(
    full_prompt: &str,
    cfg: &Config,
    manual_skills: &Option<String>,
) -> Result<String, Box<dyn Error>> {
    let matching_skills = get_matching_skills(full_prompt, manual_skills)?;

    for skill in &matching_skills {
        if manual_skills.is_some() {
            eprintln!(
                "{}",
                style(format!("📚 Using skill: {}", skill.name)).cyan()
            );
        } else {
            eprintln!(
                "{}",
                style(format!("📚 Auto-detected skill: {}", skill.name))
                    .cyan()
                    .dim()
            );
        }
    }

    let os = std::env::consts::OS;
    let shell_str = cfg
        .shell
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    build_system_prompt(os, &shell_str, &matching_skills)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn mock_skill(name: &str, description: &str, content: &str, path: &str) -> Skill {
        Skill {
            name: name.to_string(),
            description: description.to_string(),
            content: content.to_string(),
            path: PathBuf::from(path),
        }
    }

    #[test]
    fn test_build_system_prompt_no_skills() {
        let result = build_system_prompt("linux", "bash", &[]).unwrap();
        assert!(result.contains("linux"));
        assert!(result.contains("bash"));
        assert!(!result.contains("Available Skills"));
    }

    #[test]
    fn test_build_system_prompt_with_skills_only_metadata() {
        let skills = vec![mock_skill(
            "curl-gen",
            "Generate curl commands",
            "very long content that should NOT appear in prompt",
            "/home/user/.config/shelly/skills/curl/SKILL.md",
        )];
        let result = build_system_prompt("macos", "zsh", &skills).unwrap();

        assert!(result.contains("macos"));
        assert!(result.contains("zsh"));
        assert!(result.contains("Available Skills"));
        assert!(result.contains("curl-gen"));
        assert!(result.contains("Generate curl commands"));
        assert!(result.contains("/home/user/.config/shelly/skills/curl/SKILL.md"));
        assert!(
            !result.contains("very long content that should NOT appear in prompt"),
            "Full skill content must NOT be embedded in system prompt"
        );
    }

    #[test]
    fn test_build_system_prompt_multiple_skills() {
        let skills = vec![
            mock_skill("a", "desc a", "content a", "/path/a/SKILL.md"),
            mock_skill("b", "desc b", "content b", "/path/b/SKILL.md"),
        ];
        let result = build_system_prompt("linux", "fish", &skills).unwrap();

        assert!(result.contains("a"));
        assert!(result.contains("b"));
        assert!(result.contains("desc a"));
        assert!(result.contains("desc b"));
        assert!(!result.contains("content a"));
        assert!(!result.contains("content b"));
    }

    #[test]
    fn test_build_system_prompt_includes_read_file_hint() {
        let skills = vec![mock_skill("x", "desc", "content", "/path/SKILL.md")];
        let result = build_system_prompt("linux", "bash", &skills).unwrap();
        assert!(result.contains("read_file"));
        assert!(result.contains("only when needed"));
    }
}
