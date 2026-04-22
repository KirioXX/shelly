use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
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

    /// Extract repo name from GitHub URL
    pub fn extract_repo_name(url: &str) -> Result<String, Box<dyn Error>> {
        let parts: Vec<&str> = url.trim_end_matches('/').split('/').collect();
        if parts.len() >= 5 {
            Ok(parts[4].to_string())
        } else {
            Err("Could not extract repo name from URL".into())
        }
    }

    /// Download repo as tarball and extract it
    pub async fn install_from_github(
        &self,
        github_url: &str,
        repo_name: &str,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let target_dir = self.skills_dir.join(repo_name);

        if target_dir.exists() {
            return Err(format!(
                "Skill '{}' is already installed. Use 'shelly skill remove {}' first.",
                repo_name, repo_name
            )
            .into());
        }

        let tarball_url = format!("{}/archive/refs/heads/main.tar.gz", github_url);

        eprintln!(
            "{}",
            style(format!("📦 Downloading skill '{}'...", repo_name)).cyan()
        );

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
        let temp_dir = std::env::temp_dir().join(format!("shelly-skill-{}", repo_name));
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir)?;
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

        let extracted_dir = &entries[0].path();

        // Validate SKILL.md exists
        let skill_md = extracted_dir.join("SKILL.md");
        if !skill_md.exists() {
            return Err(format!(
                "No SKILL.md found in repository. A valid skill requires a SKILL.md file."
            )
            .into());
        }

        // Move to skills directory
        std::fs::rename(extracted_dir, &target_dir)?;

        // Clean up temp directory
        let _ = std::fs::remove_dir_all(&temp_dir);

        eprintln!(
            "{}",
            style(format!("✓ Installed skill '{}'", repo_name)).green()
        );

        Ok(target_dir)
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
    fn test_extract_repo_name() {
        let url = "https://github.com/user/my-skill";
        let name = SkillInstaller::extract_repo_name(url).unwrap();
        assert_eq!(name, "my-skill");
    }

    #[test]
    fn test_installer_new() {
        let temp_dir = TempDir::new().unwrap();
        let installer = SkillInstaller::new(temp_dir.path().to_path_buf());
        assert_eq!(installer.skills_dir, temp_dir.path());
    }
}
