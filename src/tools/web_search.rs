use super::Tool;
use async_trait::async_trait;
use serde_json::{Value, json};

pub struct WebSearch;

#[async_trait]
impl Tool for WebSearch {
    fn name(&self) -> &str {
        "web_search"
    }

    fn description(&self) -> &str {
        "Search the web for current information. Use when the user asks about recent events, versions, or facts that might have changed."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query"
                }
            },
            "required": ["query"]
        })
    }

    async fn execute(&self, args: Value) -> Result<String, Box<dyn std::error::Error>> {
        let query = args["query"].as_str().ok_or("Missing 'query' parameter")?;

        // For MVP, use DuckDuckGo HTML or similar
        let url = format!(
            "https://html.duckduckgo.com/html/?q={}",
            urlencoding::encode(query)
        );

        let response = reqwest::get(&url).await?;
        let body = response.text().await?;

        // Basic implementation - return search was performed
        let result = format!("Search for '{}' returned {} bytes", query, body.len());

        Ok(result)
    }
}
