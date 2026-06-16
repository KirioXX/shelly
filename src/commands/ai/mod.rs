mod system_prompt;

use std::error::Error;

use console::style;

use crate::config::Config;
use crate::{ai_utils, history};
#[allow(unused_imports)]
use crate::tools::AskUser;
use crate::{APP_NAME, CONFIG_NAME};
use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionToolChoiceOption, CreateChatCompletionRequestArgs, ResponseFormat,
    ResponseFormatJsonSchema, ToolChoiceOptions,
};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct AiResponse {
    command: String,
    warning: Option<String>,
}

fn parse_ai_response(text: &str) -> Result<AiResponse, Box<dyn Error>> {
    let cleaned = strip_markdown_fences(text.trim());
    Ok(serde_json::from_str::<AiResponse>(cleaned)?)
}

/// Strip markdown code fences (```json ... ``` or ``` ... ```)
/// and return the inner content.
///
/// Handles these cases:
/// - Bare JSON (no fences) → trimmed text
/// - Fenced JSON at start → content between fences
/// - Fenced JSON with text before/after → content between fences
/// - Unclosed opening fence → everything after the fence
fn strip_markdown_fences(text: &str) -> &str {
    if let Some(start) = text.find("```") {
        let after_fence = &text[start + 3..];

        // Skip optional language tag (json, text, etc.)
        let content_start = if after_fence.starts_with("json") || after_fence.starts_with("text") {
            after_fence[4..].trim_start()
        } else {
            after_fence.trim_start()
        };

        // If the fence was at the end with nothing after it, return content before fence
        if content_start.is_empty() {
            return text[..start].trim();
        }

        // Find the closing fence
        if let Some(end) = content_start.find("```") {
            return content_start[..end].trim();
        }

        // No closing fence — return everything after the opening fence
        return content_start.trim();
    }

    text.trim()
}

fn build_schema() -> Value {
    serde_json::json!({
        "type": "object",
        "description": "A structured response containing the shell command to execute and an optional safety warning.",
        "properties": {
            "command": {
                "type": "string",
                "description": "The exact shell command to execute. Return only the command text, without markdown fences, explanations, or surrounding quotes."
            },
            "warning": {
                "type": ["string", "null"],
                "description": "A concise safety warning when the command is destructive, irreversible, may expose secrets, or otherwise needs user caution. Use null when no warning is needed."
            }
        },
        "required": ["command", "warning"],
        "additionalProperties": false
    })
}

pub async fn call(
    prompt: Vec<String>,
    dry_run: bool,
    skills: Option<String>,
) -> Result<String, Box<dyn Error>> {
    let cfg: Config = confy::load(APP_NAME, CONFIG_NAME)?;

    if cfg.api_key.is_empty() {
        return Err("API key not configured. Run 'shelly setup' first.".into());
    }

    let full_prompt = prompt.join(" ");
    let system_prompt = match system_prompt::get_system_prompt(&full_prompt, &cfg, &skills) {
        Ok(sp) => sp,
        Err(_err) => "".to_string(),
    };

    let client = ai_utils::get_client(&cfg.api_key, &cfg.api_url);
    let registry = ai_utils::create_tool_registry();
    let tools = registry.to_function_definitions();

    // Build initial messages
    let mut messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()?
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(full_prompt.clone())
            .build()?
            .into(),
    ];

    // Tool calling loop (max 3 iterations)
    let mut tool_calls_count = 0;
    let max_tool_calls = 3;
    let final_command;
    let schema = build_schema();

    loop {
        // Create a fresh spinner for each AI call.
        // We stop it before tool execution so interactive tools (ask_user)
        // get full TTY control.
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")?
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
        );
        pb.set_message("Thinking...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        // Build request - handle tools conditionally
        let request = if tool_calls_count < max_tool_calls && !tools.is_empty() {
            CreateChatCompletionRequestArgs::default()
                .max_tokens(512u32)
                .model(cfg.model.clone())
                .messages(messages.clone())
                .tools(tools.clone())
                .tool_choice(ChatCompletionToolChoiceOption::Mode(
                    ToolChoiceOptions::Auto,
                ))
                .response_format(ResponseFormat::JsonSchema {
                    json_schema: ResponseFormatJsonSchema {
                        description: Some("Shell command with optional warning".to_string()),
                        name: "shelly_command".to_string(),
                        schema: Some(schema.clone()),
                        strict: Some(true),
                    },
                })
                .build()?
        } else {
            CreateChatCompletionRequestArgs::default()
                .max_tokens(512u32)
                .model(cfg.model.clone())
                .messages(messages.clone())
                .response_format(ResponseFormat::JsonSchema {
                    json_schema: ResponseFormatJsonSchema {
                        description: Some("Shell command with optional warning".to_string()),
                        name: "shelly_command".to_string(),
                        schema: Some(schema.clone()),
                        strict: Some(true),
                    },
                })
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
                // Collect function tool calls only
                let function_tool_calls: Vec<_> = tool_calls
                    .iter()
                    .filter_map(|tc| match tc {
                        ChatCompletionMessageToolCalls::Function(fc) => Some(fc.clone()),
                        _ => None,
                    })
                    .collect();

                if !function_tool_calls.is_empty() {
                    // AI wants to call tools
                    tool_calls_count += 1;

                    // Stop the spinner before tool execution so interactive
                    // tools (ask_user) get full TTY control.
                    pb.finish_and_clear();

                    // Add assistant message with tool calls
                    let assistant_msg = ChatCompletionRequestAssistantMessageArgs::default()
                        .content(choice.message.content.clone().unwrap_or_default())
                        .tool_calls(choice.message.tool_calls.clone().unwrap_or_default())
                        .build()?;
                    messages.push(assistant_msg.into());

                    // Execute each tool call and add results
                    for tool_call in &function_tool_calls {
                        let function_call = &tool_call.function;
                        let tool_name = &function_call.name;
                        let tool_args: Value = serde_json::from_str(&function_call.arguments)?;

                        // Log tool usage similar to skills
                        eprintln!("{}", style(format!("🔧 Using tool: {}", tool_name)).cyan());

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
            // Max tool calls reached or no function tools to call
            final_command = choice.message.content.clone();
            break;
        } else {
            // AI gave direct response
            final_command = choice.message.content.clone();
            break;
        }
    }

    let raw = final_command
        .ok_or("AI returned no command")?
        .trim()
        .to_string();

    if raw.is_empty() {
        return Err("AI returned empty command. Please try with a different prompt.".into());
    }

    let parsed = parse_ai_response(&raw)?;
    let command = parsed.command.trim().to_string();

    if command.is_empty() {
        return Err("AI returned empty command. Please try with a different prompt.".into());
    }

    if let Some(warning) = parsed.warning
        && !warning.trim().is_empty()
    {
        eprintln!(
            "{}",
            style(format!("⚠️  Warning: {}", warning.trim()))
                .red()
                .bold()
        );
    }

    if dry_run {
        eprintln!("{}", style("🔍 Dry run - Command:").yellow().bold());
        eprintln!("```");
        eprintln!("{}", style(&command).cyan());
        eprintln!("```");
        eprintln!();
        eprintln!(
            "{}",
            style("✓ Command generated (not executed)").green().bold()
        );
        let entry = history::HistoryEntry::new(&full_prompt, &command, &cfg, dry_run);
        if let Err(e) = history::append(&entry) {
            eprintln!(
                "{}",
                style(format!("⚠️  Failed to save to history: {}", e)).yellow()
            );
        }
        Ok(String::new()) // Return empty so shell doesn't inject
    } else {
        eprintln!("{}", style("✓ Command generated").green().bold());
        let entry = history::HistoryEntry::new(&full_prompt, &command, &cfg, dry_run);
        if let Err(e) = history::append(&entry) {
            eprintln!(
                "{}",
                style(format!("⚠️  Failed to save to history: {}", e)).yellow()
            );
        }
        // Command goes to stdout for shell injection, no extra formatting needed
        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_command_with_warning() {
        let result =
            parse_ai_response(r#"{"command": "rm -rf /", "warning": "dangerous"}"#).unwrap();
        assert_eq!(result.command, "rm -rf /");
        assert_eq!(result.warning, Some("dangerous".to_string()));
    }

    #[test]
    fn test_parse_json_command_with_null_warning() {
        let result = parse_ai_response(r#"{"command": "echo hello", "warning": null}"#).unwrap();
        assert_eq!(result.command, "echo hello");
        assert_eq!(result.warning, None);
    }

    #[test]
    fn test_parse_rejects_plain_command() {
        assert!(parse_ai_response("ls -la").is_err());
    }

    // ---- Graceful markdown-fence handling ----
    #[test]
    fn test_parse_strips_json_fence() {
        let input = r#"```json
{"command": "ls -la", "warning": null}
```"#;
        let result = parse_ai_response(input).unwrap();
        assert_eq!(result.command, "ls -la");
        assert_eq!(result.warning, None);
    }

    #[test]
    fn test_parse_strips_plain_fence() {
        let input = r#"```
{"command": "cd /tmp", "warning": null}
```"#;
        let result = parse_ai_response(input).unwrap();
        assert_eq!(result.command, "cd /tmp");
    }

    #[test]
    fn test_parse_with_leading_trailing_whitespace_in_fence() {
        let input = r#"

```json

  {"command": "pwd", "warning": null}

```

"#;
        let result = parse_ai_response(input).unwrap();
        assert_eq!(result.command, "pwd");
    }

    #[test]
    fn test_parse_with_inline_text_around_fence() {
        let input = r#"Here is your command:
```json
{"command": "git status", "warning": null}
```
Enjoy!"#;
        let result = parse_ai_response(input).unwrap();
        assert_eq!(result.command, "git status");
    }

    // ---- strip_markdown_fences unit tests ----
    #[test]
    fn test_strip_no_fence() {
        assert_eq!(strip_markdown_fences("hello"), "hello");
    }

    #[test]
    fn test_strip_json_fence() {
        assert_eq!(strip_markdown_fences("```json\nfoo\n```"), "foo");
    }

    #[test]
    fn test_strip_plain_fence() {
        assert_eq!(strip_markdown_fences("```\nbar\n```"), "bar");
    }

    #[test]
    fn test_strip_only_open_fence() {
        assert_eq!(strip_markdown_fences("```json\nfoo"), "foo");
    }

    #[test]
    fn test_strip_closing_fence_at_end() {
        // Edge case: only a closing fence — return content before it
        assert_eq!(strip_markdown_fences("foo\n```"), "foo");
    }

    #[test]
    fn test_strip_text_before_fence() {
        assert_eq!(
            strip_markdown_fences("intro\n```json\ncontent\n```\noutro"),
            "content"
        );
    }

    #[test]
    fn test_schema_requires_nullable_warning() {
        let schema = build_schema();
        assert_eq!(
            schema["required"],
            serde_json::json!(["command", "warning"])
        );
        assert_eq!(
            schema["properties"]["warning"]["type"],
            serde_json::json!(["string", "null"])
        );
        assert!(schema["properties"]["command"]["description"].is_string());
        assert!(schema["properties"]["warning"]["description"].is_string());
    }
}
