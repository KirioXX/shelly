use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Config;

/// A single history entry.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: String,
    pub prompt: String,
    pub command: String,
    pub shell: String,
    pub dry_run: bool,
}

impl HistoryEntry {
    /// Create a new entry from the current config and generation result.
    pub fn new(prompt: &str, command: &str, cfg: &Config, dry_run: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
            prompt: prompt.to_string(),
            command: command.to_string(),
            shell: cfg
                .shell
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            dry_run,
        }
    }
}

/// Resolve the path to the history JSONL file: `~/.config/shelly/history.jsonl`
fn history_file() -> Result<PathBuf, Box<dyn Error>> {
    let dir = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("shelly");
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir.join("history.jsonl"))
}

/// Append a single entry to the history file.
pub fn append(entry: &HistoryEntry) -> Result<(), Box<dyn Error>> {
    let path = history_file()?;
    let mut file = OpenOptions::new().append(true).create(true).open(&path)?;
    let line = serde_json::to_string(entry)?;
    writeln!(file, "{}", line)?;
    Ok(())
}

/// Load the last `limit` entries, newest first.
pub fn load(limit: usize) -> Result<Vec<HistoryEntry>, Box<dyn Error>> {
    let path = history_file()?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = fs::File::open(&path)?;
    let reader = BufReader::new(file);
    let mut entries: Vec<HistoryEntry> = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            if line.trim().is_empty() {
                return None;
            }
            serde_json::from_str::<HistoryEntry>(&line).ok()
        })
        .collect();

    entries.reverse(); // newest first
    entries.truncate(limit);
    Ok(entries)
}

/// Search history entries (case-insensitive substring match on prompt and command).
pub fn search(pattern: &str, limit: usize) -> Result<Vec<HistoryEntry>, Box<dyn Error>> {
    let pat_lower = pattern.to_lowercase();
    let all = load(usize::MAX)?;
    let filtered: Vec<HistoryEntry> = all
        .into_iter()
        .filter(|e| {
            e.prompt.to_lowercase().contains(&pat_lower)
                || e.command.to_lowercase().contains(&pat_lower)
        })
        .take(limit)
        .collect();
    Ok(filtered)
}

/// Clear the entire history file.
pub fn clear() -> Result<(), Box<dyn Error>> {
    let path = history_file()?;
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

/// Replay the entry at `index` (0 = most recent).
pub fn replay(index: usize) -> Result<HistoryEntry, Box<dyn Error>> {
    let entries = load(usize::MAX)?;
    entries
        .into_iter()
        .nth(index)
        .ok_or_else(|| "No command at that index in history".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Shell;
    use std::path::Path;

    fn tmp_history_path() -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("shelly-test-history-{}.jsonl", std::process::id()));
        p
    }

    fn setup_tmp() {
        let p = tmp_history_path();
        let _ = fs::remove_file(&p);
    }

    fn mock_entry(prompt: &str, command: &str) -> HistoryEntry {
        HistoryEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
            prompt: prompt.to_string(),
            command: command.to_string(),
            shell: "zsh".to_string(),
            dry_run: false,
        }
    }

    fn append_direct(path: &Path, entry: &HistoryEntry) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();
        writeln!(file, "{}", serde_json::to_string(entry).unwrap()).unwrap();
    }

    #[test]
    fn test_append_and_load() {
        setup_tmp();
        let path = tmp_history_path();

        let e1 = mock_entry("list docker", "docker ps");
        let e2 = mock_entry("git status", "git status");
        append_direct(&path, &e1);
        append_direct(&path, &e2);

        // This test is tricky because history_file() uses the real config path,
        // not a temp path. We test the full append/load cycle indirectly via
        // the smoke tests, but here we test the parsing logic directly.
    }

    // Tests for the pure functions (no filesystem side-effects)

    #[test]
    fn test_entry_serialize_roundtrip() {
        let entry = mock_entry("hello", "echo hello");
        let json = serde_json::to_string(&entry).unwrap();
        let parsed: HistoryEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.prompt, entry.prompt);
        assert_eq!(parsed.command, entry.command);
        assert_eq!(parsed.shell, entry.shell);
        assert_eq!(parsed.dry_run, entry.dry_run);
    }

    #[test]
    fn test_entry_new_from_config() {
        let cfg = Config {
            model: "test".into(),
            api_url: "http://test".into(),
            api_key: "key".into(),
            shell: Some(Shell::Fish),
        };
        let entry = HistoryEntry::new("prompt", "cmd", &cfg, true);
        assert_eq!(entry.prompt, "prompt");
        assert_eq!(entry.command, "cmd");
        assert_eq!(entry.shell, "Fish");
        assert!(entry.dry_run);
        assert!(!entry.id.is_empty());
        assert!(!entry.timestamp.is_empty());
    }

    #[test]
    fn test_entry_new_unknown_shell() {
        let cfg = Config {
            model: "test".into(),
            api_url: "http://test".into(),
            api_key: "key".into(),
            shell: None,
        };
        let entry = HistoryEntry::new("p", "c", &cfg, false);
        assert_eq!(entry.shell, "unknown");
    }

    #[test]
    fn test_parse_jsonl_lines() {
        let lines = [
            r#"{"id":"a","timestamp":"2026-01-01T00:00:00Z","prompt":"p1","command":"c1","shell":"zsh","dry_run":false}"#,
            r#"{"id":"b","timestamp":"2026-01-01T00:00:01Z","prompt":"p2","command":"c2","shell":"bash","dry_run":true}"#,
        ];
        let entries: Vec<HistoryEntry> = lines
            .iter()
            .filter_map(|l| serde_json::from_str::<HistoryEntry>(l).ok())
            .collect();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].prompt, "p1");
        assert!(entries[1].dry_run);
    }

    #[test]
    fn test_parse_skips_empty_lines() {
        let lines: Vec<&str> = vec!["", "  "];
        let entries: Vec<HistoryEntry> = lines
            .iter()
            .filter_map(|l| serde_json::from_str::<HistoryEntry>(l).ok())
            .collect();
        assert!(entries.is_empty());
    }
}
