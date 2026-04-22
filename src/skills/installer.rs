use std::error::Error;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use console::style;

pub struct SkillInstaller {
    skills_dir: PathBuf,
}

impl SkillInstaller {
    pub fn new(skills_dir: PathBuf) -> Self {
        Self { skills_dir }
    }

    /// Parse GitHub URL or user/repo shorthand into full URL
    pub fn parse_github_url(input: &str) -> Result<String, Box<dyn Error>> {
        if input.starts_with("https://github.com/") || input.starts_with("http://github.com/") {
            let parts: Vec<&str> = input.trim_end_matches('/').split('/').collect();
            if parts.len() >= 5 {
                let user = parts[3];
                let repo = parts[4];
                Ok(format!("https://github.com/{}/{}", user, repo))
            } else {
                Err("Invalid GitHub URL".into())
            }
        } else if input.contains('/') && !input.contains("://") {
            Ok(format!("https://github.com/{}", input))
        } else {
            Err("Expected GitHub URL or user/repo format".into())
        }
    }

    /// Download repo as tarball and extract skills
    pub async fn install_from_github(
        &self,
        github_url: &str,
        specific_skill: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let tarball_url = format!("{}/archive/refs/heads/main.tar.gz", github_url);

        eprintln!("{}", style("📦 Downloading...").cyan());

        // Download tarball
        let response = reqwest::get(&tarball_url).await?;

        let response = if !response.status().is_success() {
            // Try 'master' branch if 'main' fails
            let tarball_url = format!("{}/archive/refs/heads/master.tar.gz", github_url);
            let response = reqwest::get(&tarball_url).await?;
            if !response.status().is_success() {
                return Err(format!(
                    "Failed to download: {} (tried main and master branches)",
                    response.status()
                )
                .into());
            }
            response
        } else {
            response
        };

        let bytes = response.bytes().await?;

        // Create temp directory for extraction using std::env::temp_dir()
        let temp_dir = std::env::temp_dir().join(format!("shelly-skill-{}"
            , std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs()));
        if temp_dir.exists() {
            let _ = std::fs::remove_dir_all(&temp_dir);
        }
        std::fs::create_dir_all(&temp_dir)?;

        let tar_path = temp_dir.join("download.tar.gz");

        // Write tarball to temp file
        let mut file = std::fs::File::create(&tar_path)?;
        file.write_all(&bytes)?;

        eprintln!("{}", style("📂 Extracting...").cyan());

        // Extract tarball
        let output = Command::new("tar")
            .args([
                "-xzf",
                tar_path.to_str().unwrap(),
                "-C",
                temp_dir.to_str().unwrap(),
            ])
            .output()?;

        if !output.status.success() {
            return Err("Failed to extract tarball".into());
        }

        // Find the extracted directory (should be repo-name-branch/)
        let entries: Vec<_> = std::fs::read_dir(&temp_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path() != tar_path)
            .collect();

        if entries.is_empty() {
            return Err("No files extracted from tarball".into());
        }

        let extracted_dir = entries[0].path();

        // Find all skill directories (directories containing SKILL.md)
        let mut available_skills: Vec<(String, PathBuf)> = Vec::new();
        self.find_skills_in_dir(&extracted_dir, &mut available_skills)?;

        if available_skills.is_empty() {
            return Err(
                "No skills found in repository. Expected directories containing SKILL.md files."
                    .into(),
            );
        }

        // Filter to specific skill if requested
        let skills_to_install: Vec<(String, PathBuf)> = match specific_skill {
            Some(name) => {
                let found = available_skills
                    .iter()
                    .find(|(skill_name, _)| skill_name == name || skill_name == &name.replace("-", "_"))
                    .cloned();
                match found {
                    Some(skill) => vec![skill],
                    None => {
                        return Err(format!(
                            "Skill '{}' not found. Available skills: {}",
                            name,
                            available_skills.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join(", ")
                        )
                        .into());
                    }
                }
            }
            None => available_skills,
        };

        // Install each skill
        let mut installed = Vec::new();
        for (skill_name, skill_path) in skills_to_install {
            let target_dir = self.skills_dir.join(&skill_name);

            if target_dir.exists() {
                eprintln!(
                    "{}",
                    style(format!("⚠️  Skill '{}' already installed, skipping", skill_name)).yellow()
                );
                continue;
            }

            eprintln!(
                "{}",
                style(format!("  📁 Installing '{}'...", skill_name)).dim()
            );

            // Move to skills directory
            std::fs::rename(&skill_path, &target_dir)?;

            // Add metadata
            let meta_path = target_dir.join(".skill-source.json");
            let _ = std::fs::write(
                meta_path,
                format!(
                    r#"{{"source": "{}", "installedAt": "{}"}}"#,
                    github_url,
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs()
                ),
            );

            installed.push(skill_name);
        }

        // Clean up temp directory
        let _ = std::fs::remove_dir_all(&temp_dir);

        Ok(installed)
    }

    /// Recursively find all skill directories (containing SKILL.md)
    fn find_skills_in_dir(
        &self,
        dir: &Path,
        skills: &mut Vec<(String, PathBuf)>,
    ) -> Result<(), Box<dyn Error>> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let skill_md = path.join("SKILL.md");
                if skill_md.exists() {
                    // This is a skill directory - extract name from SKILL.md
                    if let Some(name) = self.extract_skill_name(&skill_md)? {
                        skills.push((name, path));
                    }
                } else {
                    // Check if this directory contains skill subdirectories
                    // but don't recurse into common non-skill directories
                    let dir_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");
                    if !matches!(dir_name, "node_modules" | ".git" | "target" | ".github" | "tests" | "docs") {
                        self.find_skills_in_dir(&path, skills)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Extract skill name from SKILL.md frontmatter
    fn extract_skill_name(&self, path: &Path) -> Result<Option<String>, Box<dyn Error>> {
        let content = std::fs::read_to_string(path)?;

        // Parse frontmatter for name field
        let mut in_frontmatter = false;
        let mut _frontmatter_done = false;

        for line in content.lines() {
            if line == "---" {
                if !in_frontmatter {
                    in_frontmatter = true;
                } else {
                    _frontmatter_done = true;
                    break;
                }
                continue;
            }

            if in_frontmatter && let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                if key == "name" && !value.is_empty() {
                    return Ok(Some(value.to_string()));
                }
            }
        }

        // Fallback: use directory name
        Ok(path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_full_github_url() {
        let url = "https://github.com/user/my-skill";
        let result = SkillInstaller::parse_github_url(url).unwrap();
        assert_eq!(result, "https://github.com/user/my-skill");
    }

    #[test]
    fn test_parse_shorthand() {
        let url = "user/my-skill";
        let result = SkillInstaller::parse_github_url(url).unwrap();
        assert_eq!(result, "https://github.com/user/my-skill");
    }

    #[test]
    fn test_parse_url_with_trailing_slash() {
        let url = "https://github.com/user/my-skill/";
        let result = SkillInstaller::parse_github_url(url).unwrap();
        assert_eq!(result, "https://github.com/user/my-skill");
    }

    #[test]
    fn test_installer_new() {
        let temp_dir = TempDir::new().unwrap();
        let installer = SkillInstaller::new(temp_dir.path().to_path_buf());
        assert_eq!(installer.skills_dir, temp_dir.path());
    }
}
