use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

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
    
    pub fn list(&self) -> Vec<&dyn Tool> {
        self.tools.values().map(|t| t.as_ref()).collect()
    }
    
    /// Generate function definitions for OpenAI
    pub fn to_function_definitions(&self) -> Vec<async_openai::types::ChatCompletionFunctions> {
        self.tools.values()
            .map(|tool| {
                async_openai::types::ChatCompletionFunctions {
                    name: tool.name().to_string(),
                    description: Some(tool.description().to_string()),
                    parameters: tool.parameters(),
                }
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
