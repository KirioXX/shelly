use std::error::Error;
use std::collections::BTreeMap;
use console::style;
use handlebars::Handlebars;
use indicatif::{ProgressBar, ProgressStyle};
use crate::{APP_NAME, CONFIG_NAME};
use crate::config::{Config};
use crate::skills::SkillManager;
use crate::tools::{ToolRegistry, WebSearch, ReadFile};
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs
};
use async_openai::{
    config::OpenAIConfig,
    Client,
};
use serde_json::Value;


const SYSTEM_PROMPT_TEMPLATE: &str = include_str!("prompts/system-prompt.md");

fn get_client(api_key: &str, api_base: &str) -> Client<OpenAIConfig>{
  Client::with_config(
        OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(api_base),
    )
}

fn create_tool_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(WebSearch);
    registry.register(ReadFile);
    registry
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
    let registry = create_tool_registry();
    let tools = registry.to_function_definitions();
    
    // Build initial messages
    let mut messages: Vec<async_openai::types::ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()?
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(full_prompt)
            .build()?
            .into(),
    ];

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")?
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
    );
    pb.set_message("Thinking...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // Tool calling loop (max 3 iterations)
    let mut tool_calls_count = 0;
    let max_tool_calls = 3;
    let mut final_command = None;

    loop {
        // Build request - handle tools conditionally
        let request = if tool_calls_count < max_tool_calls && !tools.is_empty() {
            CreateChatCompletionRequestArgs::default()
                .max_tokens(512u32)
                .model(cfg.model.clone())
                .messages(messages.clone())
                .tools(tools.clone())
                .tool_choice("auto")
                .build()?
        } else {
            CreateChatCompletionRequestArgs::default()
                .max_tokens(512u32)
                .model(cfg.model.clone())
                .messages(messages.clone())
                .build()?
        };
        let response = client.chat().create(request).await?;

        if response.choices.is_empty() {
            pb.finish_and_clear();
            return Err("AI returned no response. Please try again.".into());
        }

        let choice = response.choices.first().unwrap();
        
        // Check if AI wants to call tools
        if let Some(tool_calls) = &choice.message.tool_calls {
            if !tool_calls.is_empty() && tool_calls_count < max_tool_calls {
                // AI wants to call tools
                tool_calls_count += 1;
                pb.set_message(format!("Using tools ({}/{})...", tool_calls_count, max_tool_calls));
                
                // Add assistant message with tool calls
                let assistant_msg = async_openai::types::ChatCompletionRequestAssistantMessageArgs::default()
                    .content(choice.message.content.clone().unwrap_or_default())
                    .tool_calls(choice.message.tool_calls.clone().unwrap_or_default())
                    .build()?;
                messages.push(assistant_msg.into());
                
                // Execute each tool call and add results
                for tool_call in tool_calls {
                    let function_call = &tool_call.function;
                    let tool_name = &function_call.name;
                    let tool_args: Value = serde_json::from_str(&function_call.arguments)?;
                    
                    // Execute the tool
                    let result = if let Some(tool) = registry.get(tool_name) {
                        match tool.execute(tool_args).await {
                            Ok(output) => output,
                            Err(e) => format!("Error: {}", e),
                        }
                    } else {
                        format!("Error: Tool '{}' not found", tool_name)
                    };
                    
                    // Add tool result to messages
                    messages.push(
                        async_openai::types::ChatCompletionRequestToolMessageArgs::default()
                            .content(result)
                            .tool_call_id(tool_call.id.clone())
                            .build()?
                            .into()
                    );
                }
            } else {
                // Max tool calls reached or no tools to call
                final_command = choice.message.content.clone();
                break;
            }
        } else {
            // AI gave direct response
            final_command = choice.message.content.clone();
            break;
        }
    }

    pb.finish_and_clear();

    let command = final_command
        .ok_or("AI returned no command")?
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
