use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub mod installer;

pub struct Skill {
    pub name: String,
    pub description: String,
    pub content: String,
}

pub struct SkillManager {
    skills_dir: PathBuf,
}

impl SkillManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let skills_dir = dirs::home_dir()
            .ok_or("Could not find home directory")?
            .join(".config")
            .join("shelly")
            .join("skills");

        Self::with_path(skills_dir)
    }
    
    pub fn with_path(skills_dir: PathBuf) -> Result<Self, Box<dyn Error>> {
        // Create directory if it doesn't exist
        if !skills_dir.exists() {
            fs::create_dir_all(&skills_dir)?;
        }

        Ok(Self { skills_dir })
    }

    pub async fn install_from_url(
        &self,
        url: &str,
        specific_skill: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        use crate::skills::installer::SkillInstaller;

        let github_url = SkillInstaller::parse_github_url(url)?;

        let installer = SkillInstaller::new(self.skills_dir.clone());
        installer.install_from_github(&github_url, specific_skill).await
    }

    pub fn discover_skills(&self) -> Result<Vec<Skill>, Box<dyn Error>> {
        let mut skills = Vec::new();

        if !self.skills_dir.exists() {
            return Ok(skills);
        }

        for entry in fs::read_dir(&self.skills_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let skill_md = path.join("SKILL.md");
                if skill_md.exists() && let Some(skill) = self.parse_skill(&skill_md)? {
                    skills.push(skill);
                }
            }
        }

        Ok(skills)
    }

    pub fn parse_skill(&self, path: &PathBuf) -> Result<Option<Skill>, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;

        // Parse frontmatter (simple YAML-like parsing)
        let mut name = String::new();
        let mut description = String::new();
        let mut in_frontmatter = false;
        let mut frontmatter_done = false;
        let mut body_lines = Vec::new();

        for line in content.lines() {
            if line == "---" {
                if !in_frontmatter {
                    in_frontmatter = true;
                } else {
                    frontmatter_done = true;
                    in_frontmatter = false;
                }
                continue;
            }

            if in_frontmatter {
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim();
                    let value = value.trim().to_string();
                    match key {
                        "name" => name = value,
                        "description" => description = value,
                        _ => {}
                    }
                }
            } else if frontmatter_done {
                body_lines.push(line);
            }
        }

        if name.is_empty() {
            return Ok(None);
        }

        Ok(Some(Skill {
            name,
            description,
            content: body_lines.join("\n"),
        }))
    }

    pub fn find_matching_skill(&self, prompt: &str) -> Result<Option<Skill>, Box<dyn Error>> {
        let prompt_lower = prompt.to_lowercase();
        let skills = self.discover_skills()?;

        for skill in skills {
            // Simple keyword matching
            let keywords: Vec<String> = skill.description.to_lowercase()
                .split_whitespace()
                .filter(|w| w.len() > 3)
                .map(|w| w.to_string())
                .collect();

            let matches = keywords.iter()
                .any(|kw| prompt_lower.contains(kw));

            if matches {
                return Ok(Some(skill));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_skill_manager_with_path() {
        let temp_dir = TempDir::new().unwrap();
        let skills_path = temp_dir.path().join("skills");
        
        let manager = SkillManager::with_path(skills_path.clone()).unwrap();
        assert_eq!(manager.skills_dir, skills_path);
    }

    #[test]
    fn test_parse_valid_skill() {
        let temp_dir = TempDir::new().unwrap();
        let skill_file = temp_dir.path().join("SKILL.md");
        
        let skill_content = r#"---
name: test-skill
description: Use when testing
---

# Test Skill

This is a test skill.
"#;
        
        let mut file = std::fs::File::create(&skill_file).unwrap();
        file.write_all(skill_content.as_bytes()).unwrap();
        
        let manager = SkillManager::with_path(temp_dir.path().to_path_buf()).unwrap();
        let skill = manager.parse_skill(&skill_file).unwrap();
        
        assert!(skill.is_some());
        let skill = skill.unwrap();
        assert_eq!(skill.name, "test-skill");
        assert_eq!(skill.description, "Use when testing");
        assert!(skill.content.contains("# Test Skill"));
    }

    #[test]
    fn test_parse_invalid_skill_no_frontmatter() {
        let temp_dir = TempDir::new().unwrap();
        let skill_file = temp_dir.path().join("SKILL.md");
        
        let mut file = std::fs::File::create(&skill_file).unwrap();
        file.write_all(b"Just content, no frontmatter.").unwrap();
        
        let manager = SkillManager::with_path(temp_dir.path().to_path_buf()).unwrap();
        let skill = manager.parse_skill(&skill_file).unwrap();
        
        assert!(skill.is_none()); // Should return None for invalid skill
    }

    #[test]
    fn test_parse_skill_missing_name() {
        let temp_dir = TempDir::new().unwrap();
        let skill_file = temp_dir.path().join("SKILL.md");
        
        let skill_content = r#"---
description: Use when testing
---

Content here.
"#;
        
        let mut file = std::fs::File::create(&skill_file).unwrap();
        file.write_all(skill_content.as_bytes()).unwrap();
        
        let manager = SkillManager::with_path(temp_dir.path().to_path_buf()).unwrap();
        let skill = manager.parse_skill(&skill_file).unwrap();
        
        assert!(skill.is_none()); // Should return None when name is missing
    }

    #[test]
    fn test_find_matching_skill() {
        let temp_dir = TempDir::new().unwrap();
        let skills_dir = temp_dir.path().join("skills");
        std::fs::create_dir(&skills_dir).unwrap();
        
        // Create a test skill directory
        let skill_dir = skills_dir.join("test-skill");
        std::fs::create_dir(&skill_dir).unwrap();
        
        let skill_content = r#"---
name: test-skill
description: Use when the user mentions testing
---

# Test Skill
"#;
        
        let mut file = std::fs::File::create(skill_dir.join("SKILL.md")).unwrap();
        file.write_all(skill_content.as_bytes()).unwrap();
        
        let manager = SkillManager::with_path(skills_dir).unwrap();
        let skill = manager.find_matching_skill("I need to do some testing").unwrap();
        
        assert!(skill.is_some());
        assert_eq!(skill.unwrap().name, "test-skill");
    }

    #[test]
    fn test_find_no_matching_skill() {
        let temp_dir = TempDir::new().unwrap();
        let skills_dir = temp_dir.path().join("skills");
        std::fs::create_dir(&skills_dir).unwrap();
        
        // Create a test skill
        let skill_dir = skills_dir.join("test-skill");
        std::fs::create_dir(&skill_dir).unwrap();
        
        let skill_content = r#"---
name: test-skill
description: Use when the user mentions testing
---
"#;
        
        let mut file = std::fs::File::create(skill_dir.join("SKILL.md")).unwrap();
        file.write_all(skill_content.as_bytes()).unwrap();
        
        let manager = SkillManager::with_path(skills_dir).unwrap();
        
        // This prompt doesn't match any keywords
        let skill = manager.find_matching_skill("deploy to production").unwrap();
        
        assert!(skill.is_none());
    }
}
