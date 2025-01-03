use crate::error::{ChirpifyError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const DEFAULT_SYSTEM_PROMPT_PATH: &str = "SYSTEM_PROMPT.md";

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

pub struct TweetAgent {
    client: Client,
    api_key: String,
}

impl TweetAgent {
    pub fn new() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| ChirpifyError::ConfigError("OPENAI_API_KEY not set".into()))?;

        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    fn read_system_prompt() -> Result<String> {
        let custom_path =
            env::var("CHIRPFY_PROMPT_PATH").unwrap_or(DEFAULT_SYSTEM_PROMPT_PATH.to_string());
        let path = Path::new(&custom_path);

        fs::read_to_string(path)
            .map_err(|e| ChirpifyError::ConfigError(format!("Failed to read system prompt: {}", e)))
    }

    pub async fn refine(&self, tweet: &str) -> Result<String> {
        let system_prompt = Self::read_system_prompt()?;

        let request = ChatRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                Message {
                    role: "user".to_string(),
                    content: tweet.to_string(),
                },
            ],
            temperature: 0.7,
        };

        let response = self
            .client
            .post(OPENAI_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| ChirpifyError::OpenAIError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ChirpifyError::OpenAIError(format!(
                "API request failed: {}",
                response.status()
            )));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| ChirpifyError::OpenAIError(e.to_string()))?;

        chat_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| ChirpifyError::OpenAIError("No response from OpenAI".into()))
    }
}
