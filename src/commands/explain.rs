use std::error::Error;

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use async_openai::types::chat::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};

use crate::config::Config;
use crate::{APP_NAME, CONFIG_NAME};

const EXPLAIN_SYSTEM_PROMPT: &str = r#"You are a helpful shell command explainer. 

Given a shell command, break it down and explain what it does in plain English. 
Cover:
- What the overall command accomplishes
- What each flag/option means
- Any potential side effects or risks
- When you would typically use this command

Be concise but thorough. Use bullet points for flags. Mention if the command is destructive or irreversible.
"#;

fn get_client(api_key: &str, api_base: &str) -> async_openai::Client<async_openai::config::OpenAIConfig> {
    async_openai::Client::with_config(
        async_openai::config::OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(api_base),
    )
}

pub async fn explain(command: Vec<String>) -> Result<(), Box<dyn Error>> {
    let cfg: Config = confy::load(APP_NAME, CONFIG_NAME)?;

    if cfg.api_key.is_empty() {
        return Err("API key not configured. Run 'shelly setup' first.".into());
    }

    let full_command = command.join(" ");
    if full_command.is_empty() {
        return Err("No command provided. Usage: shelly explain <command>".into());
    }

    let client = get_client(&cfg.api_key, &cfg.api_url);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")?
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
    );
    pb.set_message("Analyzing command...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let user_prompt = format!(
        "Explain this shell command step by step:\n\n```bash\n{}\n```",
        full_command
    );

    let messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(EXPLAIN_SYSTEM_PROMPT)
            .build()?
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(user_prompt)
            .build()?
            .into(),
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(1024u32)
        .model(cfg.model.clone())
        .messages(messages)
        .build()?;

    let response = match client.chat().create(request).await {
        Ok(resp) => resp,
        Err(e) => {
            pb.finish_and_clear();
            eprintln!("Debug: API Error details: {:?}", e);
            return Err(format!("API request failed: {:?}", e).into());
        }
    };

    pb.finish_and_clear();

    if response.choices.is_empty() {
        return Err("AI returned no response. Please try again.".into());
    }

    let explanation = response
        .choices
        .first()
        .unwrap()
        .message
        .content
        .clone()
        .unwrap_or_default()
        .trim()
        .to_string();

    if explanation.is_empty() {
        return Err("AI returned empty explanation. Please try again.".into());
    }

    println!(
        "\n{} {}\n",
        style("🐚").cyan(),
        style(format!("Explaining: {}", full_command)).bold()
    );

    println!("{}", explanation);

    println!(
        "\n{}",
        style("─".repeat(50)).dim()
    );

    Ok(())
}
