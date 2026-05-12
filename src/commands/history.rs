use std::error::Error;

use dialoguer::{console::Style, theme::ColorfulTheme, Confirm};

use crate::history;

/// Display history entries in a human-readable table.
pub fn history(
    limit: usize,
    search: Option<String>,
    clear: bool,
    raw: bool,
) -> Result<(), Box<dyn Error>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    if clear {
        if Confirm::with_theme(&theme)
            .with_prompt("Are you sure you want to clear your entire command history?")
            .default(false)
            .interact()?
        {
            history::clear()?;
            println!("✓ History cleared");
        } else {
            println!("History left unchanged.");
        }
        return Ok(());
    }

    let entries = match search {
        Some(pat) => history::search(&pat, limit)?,
        None => history::load(limit)?,
    };

    if entries.is_empty() {
        println!("No history entries found.");
        return Ok(());
    }

    if raw {
        for entry in &entries {
            println!("{}", serde_json::to_string(entry)?);
        }
        return Ok(());
    }

    // Human-readable table
    println!("{}", format_history_table(&entries));
    Ok(())
}

/// Format a slice of entries as a numbered table.
fn format_history_table(entries: &[history::HistoryEntry]) -> String {
    let mut lines = vec!["Recent commands:\n".to_string()];
    for (i, entry) in entries.iter().enumerate() {
        let dt = entry
            .timestamp
            .replace("T", " ")
            .split('.')
            .next()
            .unwrap_or(&entry.timestamp)
            .to_string();
        let dry = if entry.dry_run { " [dry-run]" } else { "" };
        lines.push(format!(
            "  {:3}. {}  \"{}\"{}\n       →  {}",
            i + 1,
            dt,
            entry.prompt,
            dry,
            entry.command
        ));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::HistoryEntry;

    fn mock_entry(prompt: &str, command: &str, dry: bool) -> HistoryEntry {
        HistoryEntry {
            id: "id".into(),
            timestamp: "2026-05-06T14:32:01Z".into(),
            prompt: prompt.into(),
            command: command.into(),
            shell: "zsh".into(),
            dry_run: dry,
        }
    }

    #[test]
    fn test_format_table_empty() {
        assert_eq!(format_history_table(&[]), "Recent commands:\n");
    }

    #[test]
    fn test_format_table_one() {
        let entries = vec![mock_entry("list docker", "docker ps", false)];
        let out = format_history_table(&entries);
        assert!(out.contains("list docker"));
        assert!(out.contains("docker ps"));
        assert!(out.contains("2026-05-06 14:32:01"));
    }

    #[test]
    fn test_format_table_dry_run() {
        let entries = vec![mock_entry("test", "echo hi", true)];
        let out = format_history_table(&entries);
        assert!(out.contains("[dry-run]"));
    }

    #[test]
    fn test_format_table_multiple() {
        let entries = vec![
            mock_entry("a", "cmd-a", false),
            mock_entry("b", "cmd-b", false),
        ];
        let out = format_history_table(&entries);
        assert!(out.contains("  1."));
        assert!(out.contains("  2."));
        assert!(out.contains("cmd-a"));
        assert!(out.contains("cmd-b"));
    }
}
