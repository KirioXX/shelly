use async_openai::{Client, config::OpenAIConfig};

use crate::{AskUser, ReadFile, ToolRegistry, WebSearch};

pub fn get_client(api_key: &str, api_base: &str) -> Client<OpenAIConfig> {
    Client::with_config(
        OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(api_base),
    )
}

pub fn create_tool_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(WebSearch);
    registry.register(ReadFile);
    registry.register(AskUser);
    registry
}
