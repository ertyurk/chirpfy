use crate::error::{ChirpifyError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const SYSTEM_PROMPT: &str = r#"You are a tweet refiner for a technical product leader. Transform inputs into impactful tweets that:
1. Emphasize technical insights and product thinking
2. Maintain professional credibility while being approachable
3. Include contrarian views when relevant
4. Focus on systems thinking and scalability
5. Keep the entrepreneurial angle
6. Use emojis sparingly (max 1-2) and only when they add value

Style Guidelines:
- Sharp and direct
- Technical but not overly academic
- Product-focused
- Slightly provocative when appropriate
- For long posts (>280 chars):
  - Break into clear paragraphs
  - Use bullet points for lists
  - Keep structure clean and readable
  - Maintain focus despite length

Avoid:
- Generic startup platitudes
- Overly promotional language
- Hashtags
- Thread suggestions
- Unnecessary verbosity (even in long posts)

Return ONLY the refined tweet/post, nothing else."#;

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

    pub async fn refine(&self, tweet: &str) -> Result<String> {
        let request = ChatRequest {
            model: "gpt-4".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: SYSTEM_PROMPT.to_string(),
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
