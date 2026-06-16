use std::error::Error;

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionToolChoiceOption, CreateChatCompletionRequestArgs, ToolChoiceOptions,
};

use crate::config::Config;
use crate::{APP_NAME, CONFIG_NAME, ai_utils};

const EXPLAIN_SYSTEM_PROMPT: &str = r#"You are a helpful shell command explainer.

Given a shell command, break it down and explain what it does in plain English.
Cover:
- What the overall command accomplishes
- What each flag/option means
- Any potential side effects or risks
- When you would typically use this command

Be concise but thorough. Use bullet points for flags. Mention if the command is destructive or irreversible.
"#;

pub async fn explain(command: Vec<String>) -> Result<(), Box<dyn Error>> {
    let cfg: Config = confy::load(APP_NAME, CONFIG_NAME)?;

    if cfg.api_key.is_empty() {
        return Err("API key not configured. Run 'shelly setup' first.".into());
    }

    let full_command = command.join(" ");
    if full_command.is_empty() {
        return Err("No command provided. Usage: shelly explain <command>".into());
    }

    let client = ai_utils::get_client(&cfg.api_key, &cfg.api_url);
    let registry = ai_utils::create_tool_registry();
    let tools = registry.to_function_definitions();

    let mut messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(EXPLAIN_SYSTEM_PROMPT)
            .build()?
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(format!(
                "Explain this shell command step by step:\n\n```bash\n{}\n```",
                full_command
            ))
            .build()?
            .into(),
    ];

    // Tool calling loop (max 3 iterations)
    let mut tool_calls_count = 0;
    let max_tool_calls = 3;
    let final_explanation;

    loop {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")?
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
        );
        pb.set_message("Analyzing command...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        let request = if tool_calls_count < max_tool_calls && !tools.is_empty() {
            CreateChatCompletionRequestArgs::default()
                .max_tokens(1024u32)
                .model(cfg.model.clone())
                .messages(messages.clone())
                .tools(tools.clone())
                .tool_choice(ChatCompletionToolChoiceOption::Mode(
                    ToolChoiceOptions::Auto,
                ))
                .build()?
        } else {
            CreateChatCompletionRequestArgs::default()
                .max_tokens(1024u32)
                .model(cfg.model.clone())
                .messages(messages.clone())
                .build()?
        };

        let response = match client.chat().create(request).await {
            Ok(resp) => resp,
            Err(e) => {
                pb.finish_and_clear();
                eprintln!("Debug: API Error details: {:?}", e);
                return Err(format!("API request failed: {:?}", e).into());
            }
        };

        if response.choices.is_empty() {
            pb.finish_and_clear();
            return Err("AI returned no response. Please try again.".into());
        }

        let choice = response.choices.first().unwrap();

        // Check if AI wants to call tools
        if let Some(tool_calls) = &choice.message.tool_calls {
            if !tool_calls.is_empty() && tool_calls_count < max_tool_calls {
                let function_tool_calls: Vec<_> = tool_calls
                    .iter()
                    .filter_map(|tc| match tc {
                        ChatCompletionMessageToolCalls::Function(fc) => Some(fc.clone()),
                        _ => None,
                    })
                    .collect();

                if !function_tool_calls.is_empty() {
                    tool_calls_count += 1;
                    pb.finish_and_clear();

                    let assistant_msg = ChatCompletionRequestAssistantMessageArgs::default()
                        .content(choice.message.content.clone().unwrap_or_default())
                        .tool_calls(choice.message.tool_calls.clone().unwrap_or_default())
                        .build()?;
                    messages.push(assistant_msg.into());

                    for tool_call in &function_tool_calls {
                        let function_call = &tool_call.function;
                        let tool_name = &function_call.name;
                        let tool_args: serde_json::Value =
                            serde_json::from_str(&function_call.arguments)?;

                        eprintln!("{}", style(format!("🔧 Using tool: {}", tool_name)).cyan());

                        let result = if let Some(tool) = registry.get(tool_name) {
                            match tool.execute(tool_args).await {
                                Ok(output) => output,
                                Err(e) => format!("Error: {}", e),
                            }
                        } else {
                            format!("Error: Tool '{}' not found", tool_name)
                        };

                        messages.push(
                            ChatCompletionRequestToolMessageArgs::default()
                                .content(result)
                                .tool_call_id(tool_call.id.clone())
                                .build()?
                                .into(),
                        );
                    }
                    continue;
                }
            }
            final_explanation = choice.message.content.clone();
            pb.finish_and_clear();
            break;
        } else {
            final_explanation = choice.message.content.clone();
            pb.finish_and_clear();
            break;
        }
    }

    let explanation = final_explanation
        .ok_or("AI returned no explanation")?
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

    println!("\n{}", style("─".repeat(50)).dim());

    Ok(())
}
