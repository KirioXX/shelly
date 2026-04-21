use async_trait::async_trait;
use serde_json::{json, Value};
use dialoguer::Select;
use console::style;
use super::Tool;

pub struct AskUser;

#[async_trait]
impl Tool for AskUser {
    fn name(&self) -> &str {
        "ask_user"
    }
    
    fn description(&self) -> &str {
        "When the user's request is ambiguous or could be interpreted in multiple ways, present options using a selectable list and ask the user to choose. Use this instead of guessing what the user wants."
    }
    
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "question": {
                    "type": "string",
                    "description": "The clarification question to display above the options"
                },
                "options": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "label": {
                                "type": "string",
                                "description": "Short display text for this option (shown in the Select list)"
                            },
                            "value": {
                                "type": "string",
                                "description": "The value to return when this option is selected"
                            }
                        },
                        "required": ["label", "value"]
                    },
                    "minItems": 2,
                    "maxItems": 5,
                    "description": "2-5 options presented as a Select list with arrow key navigation"
                }
            },
            "required": ["question", "options"]
        })
    }
    
    async fn execute(&self, args: Value) -> Result<String, Box<dyn std::error::Error>> {
        let question = args["question"].as_str()
            .ok_or("Missing 'question' parameter")?;
        
        let options = args["options"].as_array()
            .ok_or("Missing 'options' parameter")?;
        
        if options.len() < 2 {
            return Err("Need at least 2 options".into());
        }
        
        // Extract labels for Select display
        let labels: Vec<String> = options.iter()
            .map(|opt| opt["label"].as_str().unwrap_or("Unknown").to_string())
            .collect();
        
        // Show the question
        eprintln!("\n{}", style("🤔 The AI needs clarification:").yellow().bold());
        eprintln!("{}", style(question).cyan());
        eprintln!("{}", style("Navigate with ↑↓ and press Enter to select:").dim());
        
        // Present options using dialoguer::Select
        let selection = Select::new()
            .items(&labels)
            .default(0)
            .interact()?;
        
        // Get the value for the selected option
        let selected_value = options[selection]["value"]
            .as_str()
            .ok_or("Selected option missing 'value'")?;
        
        // Return the user's selection for the AI to use
        Ok(selected_value.to_string())
    }
}
