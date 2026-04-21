use std::error::Error;
use std::collections::BTreeMap;
use console::style;
use handlebars::Handlebars;
use indicatif::{ProgressBar, ProgressStyle};
use crate::{APP_NAME, CONFIG_NAME};
use crate::config::{Config};
use crate::skills::SkillManager;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs
};
use async_openai::{
    config::OpenAIConfig,
    Client,
};


const SYSTEM_PROMPT_TEMPLATE: &str = include_str!("prompts/system-prompt.md");

fn get_client(api_key: &str, api_base: &str) -> Client<OpenAIConfig>{
  Client::with_config(
        OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(api_base),
    )
}

fn get_system_prompt(full_prompt: &str, cfg: &Config) -> Result<String, Box<dyn Error>> {
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

pub async fn call(prompt: Vec<String>, dry_run: bool) -> Result<String, Box<dyn Error>> {
    // Setup
    let cfg: Config = confy::load(APP_NAME, CONFIG_NAME)?;

    if cfg.api_key.is_empty() {
        return Err("API key not configured. Run 'shelly setup' first.".into());
    }

    let full_prompt = prompt.join(" ");
    let system_prompt = match get_system_prompt(&full_prompt, &cfg) {
        Ok(sp) => sp,
        Err(_err) => "".to_string(),
    };

    let client = get_client(&cfg.api_key, &cfg.api_url);
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model(cfg.model)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system_prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(full_prompt)
                .build()?
                .into(),
        ])
        .build()?;

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")?
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
    );
    pb.set_message("Thinking...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let response = client.chat().create(request).await?;

    pb.finish_and_clear();

    if response.choices.is_empty() {
        return Err("AI returned no response. Please try again.".into());
    }

    // Extract the command from the first choice
    let command = response.choices
        .first()
        .and_then(|choice| choice.message.content.clone())
        .ok_or("AI returned empty content")?
        .trim()
        .to_string();

    if command.is_empty() {
        return Err("AI returned empty command. Please try with a different prompt.".into());
    }

    if dry_run {
        eprintln!("{}", style("🔍 Dry run - Command:").yellow().bold());
        eprintln!("```");
        eprintln!("{}", style(&command).cyan());
        eprintln!("```");
        eprintln!();
        eprintln!("{}", style("✓ Command generated (not executed)").green().bold());
        Ok(String::new())  // Return empty so shell doesn't inject
    } else {
        eprintln!("{}", style("✓ Command generated").green().bold());
        // Command goes to stdout for shell injection, no extra formatting needed
        Ok(command)
    }
}
