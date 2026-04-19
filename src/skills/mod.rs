use std::error::Error;
use std::fs;
use std::path::PathBuf;

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

        // Create directory if it doesn't exist
        if !skills_dir.exists() {
            fs::create_dir_all(&skills_dir)?;
        }

        Ok(Self { skills_dir })
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
                if skill_md.exists() {
                    if let Some(skill) = self.parse_skill(&skill_md)? {
                        skills.push(skill);
                    }
                }
            }
        }

        Ok(skills)
    }

    fn parse_skill(&self, path: &PathBuf) -> Result<Option<Skill>, Box<dyn Error>> {
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
