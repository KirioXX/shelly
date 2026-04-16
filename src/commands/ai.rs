use std::error::Error;
use crate::{APP_NAME, CONFIG_NAME};
use crate::config::{Config};
use async_openai::types::{CreateCompletionRequestArgs};
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

  // Setup Request
  let request = CreateCompletionRequestArgs::default()
        .model(cfg.model)
        .prompt(prompt)
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.completions().create(request).await?;

    println!("\nResponse (multiple):\n");
    for choice in response.choices {
        println!("{}", choice.text);
    }

  Ok("".into())
}
