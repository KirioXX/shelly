use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use async_openai::types::chat::ChatCompletionTools;
use async_openai::types::chat::FunctionObject;

/// A tool that the AI can call
#[async_trait]
pub trait Tool: Send + Sync {
    /// Tool name (used in function calling)
    fn name(&self) -> &str;
    
    /// Tool description (shown to AI)
    fn description(&self) -> &str;
    
    /// JSON schema for tool parameters
    fn parameters(&self) -> Value;
    
    /// Execute the tool with given arguments
    async fn execute(&self, args: Value) -> Result<String, Box<dyn std::error::Error>>;
}

/// Registry of available tools
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }
    
    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        self.tools.insert(tool.name().to_string(), Box::new(tool));
    }
    
    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|t| t.as_ref())
    }
    
    /// Generate function definitions for OpenAI
    pub fn to_function_definitions(&self) -> Vec<ChatCompletionTools> {
        self.tools.values()
            .map(|tool| {
                let function = FunctionObject {
                    name: tool.name().to_string(),
                    description: Some(tool.description().to_string()),
                    parameters: Some(tool.parameters()),
                    strict: None,
                };
                ChatCompletionTools::Function(
                    async_openai::types::chat::ChatCompletionTool { function }
                )
            })
            .collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub mod web_search;
pub use web_search::WebSearch;

pub mod read_file;
pub use read_file::ReadFile;

pub mod ask_user;
pub use ask_user::AskUser;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct MockTool {
        name: String,
    }

    #[async_trait]
    impl Tool for MockTool {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            "A mock tool for testing"
        }

        fn parameters(&self) -> Value {
            json!({"type": "object", "properties": {}})
        }

        async fn execute(&self, _args: Value) -> Result<String, Box<dyn std::error::Error>> {
            Ok(format!("Executed {}", self.name))
        }
    }

    #[test]
    fn test_tool_registry_new() {
        let registry = ToolRegistry::new();
        assert!(registry.get("test").is_none());
    }

    #[test]
    fn test_tool_registry_register_and_get() {
        let mut registry = ToolRegistry::new();
        registry.register(MockTool { name: "test_tool".to_string() });
        
        let tool = registry.get("test_tool");
        assert!(tool.is_some());
        assert_eq!(tool.unwrap().name(), "test_tool");
    }

    #[test]
    fn test_tool_registry_get_nonexistent() {
        let registry = ToolRegistry::new();
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_tool_registry_to_function_definitions() {
        let mut registry = ToolRegistry::new();
        registry.register(MockTool { name: "tool1".to_string() });
        registry.register(MockTool { name: "tool2".to_string() });
        
        let defs = registry.to_function_definitions();
        assert_eq!(defs.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_tool_execution() {
        let tool = MockTool { name: "test".to_string() };
        let result = tool.execute(json!({})).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Executed test");
    }
}
