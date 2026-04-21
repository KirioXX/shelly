use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::fs;
use super::Tool;

pub struct ReadFile;

#[async_trait]
impl Tool for ReadFile {
    fn name(&self) -> &str {
        "read_file"
    }
    
    fn description(&self) -> &str {
        "Read the contents of a file. Use when the user references a file or wants to see configuration."
    }
    
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file"
                }
            },
            "required": ["path"]
        })
    }
    
    async fn execute(&self, args: Value) -> Result<String, Box<dyn std::error::Error>> {
        let path = args["path"].as_str()
            .ok_or("Missing 'path' parameter")?;
        
        // Security: only allow reading within project directory or home
        let canonical = std::fs::canonicalize(path)?;
        let current_dir = std::env::current_dir()?;
        let home = dirs::home_dir().unwrap_or_else(|| current_dir.clone());
        
        if !canonical.starts_with(&current_dir) && !canonical.starts_with(&home) {
            return Err("Cannot read files outside project or home directory".into());
        }
        
        let content = fs::read_to_string(&canonical).await?;
        
        // Truncate if too large
        if content.len() > 8000 {
            Ok(format!("{}\n\n[truncated...]", &content[..8000]))
        } else {
            Ok(content)
        }
    }
}
