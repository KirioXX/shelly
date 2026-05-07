use std::error::Error;

use dialoguer::{Confirm, Input, Password, console::Style, theme::ColorfulTheme};

use crate::config::{Config, Shell};
use crate::{APP_NAME, CONFIG_NAME};

/// Display the current configuration with masked API key.
pub fn config(show_only: bool) -> Result<(), Box<dyn Error>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    let cfg: Config = confy::load(APP_NAME, CONFIG_NAME)?;

    println!("{}", format_config(&cfg));

    if show_only {
        return Ok(());
    }

    if !Confirm::with_theme(&theme)
        .with_prompt("Want to edit these settings?")
        .interact()?
    {
        println!("Config left unchanged.");
        return Ok(());
    }

    // Prompt each field, using the current value as default.
    let model: String = Input::with_theme(&theme)
        .with_prompt("AI Model")
        .default(cfg.model.clone())
        .interact()?;

    let api_url: String = Input::with_theme(&theme)
        .with_prompt("AI API Endpoint")
        .default(cfg.api_url.clone())
        .interact()?;

    let api_key = if Confirm::with_theme(&theme)
        .with_prompt("Change API key?")
        .interact()?
    {
        Password::with_theme(&theme)
            .with_prompt("AI API Key")
            .interact()?
    } else {
        cfg.api_key.clone()
    };

    let shell_options = vec!["Bash", "Zsh", "Fish"];
    let shell_index = cfg.shell.as_ref().map_or(0, |s| match s {
        Shell::Bash => 0,
        Shell::Zsh => 1,
        Shell::Fish => 2,
    });

    let shell_selection = dialoguer::Select::with_theme(&theme)
        .with_prompt("Configure shell")
        .default(shell_index)
        .items(&shell_options)
        .interact()?;

    let shell = match shell_selection {
        0 => Shell::Bash,
        1 => Shell::Zsh,
        2 => Shell::Fish,
        _ => Shell::Bash,
    };

    let new_cfg = Config {
        model,
        api_url,
        api_key,
        shell: Some(shell),
    };

    confy::store(APP_NAME, CONFIG_NAME, new_cfg)?;
    println!("✓ Config saved");
    Ok(())
}

/// Mask an API key so only a prefix and suffix remain visible.
///
/// Format: `<prefix>****<suffix>` where prefix is first N chars (up to 5)
/// and suffix is last 4 chars. Total visible characters = prefix + suffix.
/// If the key is shorter than 9 chars, it's fully masked.
pub fn mask_api_key(key: &str) -> String {
    if key.is_empty() {
        return "(not set)".to_string();
    }
    let len = key.len();
    if len <= 9 {
        let visible = len.saturating_sub(4);
        let prefix = &key[..visible.max(0)];
        format!("{}{}", prefix, "*".repeat(len - prefix.len()))
    } else {
        let prefix_len = 5.min(len.saturating_sub(4));
        let suffix_start = len.saturating_sub(4);
        let prefix = &key[..prefix_len];
        let suffix = &key[suffix_start..];
        format!("{}****{}", prefix, suffix)
    }
}

/// Format a Config into a human-readable display string.
pub fn format_config(cfg: &Config) -> String {
    let shell_str = cfg
        .shell
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "not set".to_string());

    format!(
        r#"Current configuration:

  Model:   {}
  API URL: {}
  API Key: {}
  Shell:   {}
"#,
        cfg.model,
        cfg.api_url,
        mask_api_key(&cfg.api_key),
        shell_str,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_api_key_normal() {
        assert_eq!(
            mask_api_key("sk-abcdefghijklmnopqrstuvwxyz"),
            "sk-ab****wxyz"
        );
    }

    #[test]
    fn test_mask_api_key_exact_9() {
        // "sk-ABCDEFGH" is 11 chars. For len > 9: prefix 5 + **** + suffix 4.
        assert_eq!(mask_api_key("sk-ABCDEFGH"), "sk-AB****EFGH");
    }

    #[test]
    fn test_mask_api_key_short_8() {
        // "sk-12345" is 8 chars. For len <= 9: show first (len-4) chars, then 4 asterisks.
        assert_eq!(mask_api_key("sk-12345"), "sk-1****");
    }

    #[test]
    fn test_mask_api_key_short_5() {
        assert_eq!(mask_api_key("hello"), "h****");
    }

    #[test]
    fn test_mask_api_key_4() {
        assert_eq!(mask_api_key("abcd"), "****");
    }

    #[test]
    fn test_mask_api_key_empty() {
        assert_eq!(mask_api_key(""), "(not set)");
    }

    #[test]
    fn test_format_config_full() {
        let cfg = Config {
            model: "gpt-4".into(),
            api_url: "https://api.openai.com/v1".into(),
            api_key: "sk-abcdef123456".into(),
            shell: Some(Shell::Zsh),
        };
        let output = format_config(&cfg);
        assert!(output.contains("gpt-4"));
        assert!(output.contains("https://api.openai.com/v1"));
        assert!(output.contains("sk-ab****3456"));
        assert!(output.contains("Zsh"));
    }

    #[test]
    fn test_format_config_no_shell() {
        let cfg = Config {
            model: "qwen".into(),
            api_url: "https://ollama.com/v1".into(),
            api_key: "abc".into(),
            shell: None,
        };
        let output = format_config(&cfg);
        assert!(output.contains("not set"));
    }

    #[test]
    fn test_format_config_empty_key() {
        let cfg = Config {
            model: "llama".into(),
            api_url: "https://ollama.com/v1".into(),
            api_key: String::new(),
            shell: Some(Shell::Bash),
        };
        let output = format_config(&cfg);
        assert!(output.contains("(not set)"));
    }
}
