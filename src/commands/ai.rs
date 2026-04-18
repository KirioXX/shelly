use std::error::Error;
use crate::{APP_NAME, CONFIG_NAME};
use crate::config::{Config};
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs
};
use async_openai::{
    config::OpenAIConfig,
    Client,
};

fn get_client(api_key: &str, api_base: &str) -> Client<OpenAIConfig>{
  Client::with_config(
        OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(api_base),
    )
}

pub async fn call(prompt: Vec<String>) -> Result<String, Box<dyn Error>> {
  // Setup
  let cfg: Config = confy::load(APP_NAME, CONFIG_NAME)?;
  let client = get_client(&cfg.api_key, &cfg.api_url);

  let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model(cfg.model)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt.join(" "))
                .build()?
                .into(),
        ])
        .build()?;

    eprintln!("{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    eprintln!("\nResponse:\n");
    for choice in &response.choices {
        eprintln!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    // Extract the command from the first choice
    let command = response.choices
        .first()
        .and_then(|choice| choice.message.content.clone())
        .unwrap_or_default()
        .trim()
        .to_string();

    Ok(command)
}
