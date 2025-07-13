//! An implementation of the `LlmClient` for the OpenAI API.

use super::client::{ChatMessage, LlmClient, LlmError, LlmRequest, LlmResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// OpenAI-specific request/response structures
#[derive(Serialize)]
struct OpenAiChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize)]
struct OpenAiChatResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
}

#[derive(Deserialize)]
struct OpenAiMessage {
    content: String,
}

/// A client for interacting with the OpenAI API.
pub struct OpenAiClient {
    api_key: String,
    http_client: Client,
    api_url: String,
}

impl OpenAiClient {
    /// Creates a new OpenAI client.
    /// In a real application, these values would come from a config file.
    pub fn new(api_key: String, api_url: Option<String>) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
            api_url: api_url.unwrap_or_else(|| "https://api.openai.com/v1/chat/completions".to_string()),
        }
    }
}

#[async_trait]
impl LlmClient for OpenAiClient {
    async fn execute_chat(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        let openai_request = OpenAiChatRequest {
            model: request.model,
            messages: request.messages,
        };

        let response = self
            .http_client
            .post(&self.api_url)
            .bearer_auth(&self.api_key)
            .json(&openai_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError { status, body });
        }

        let response_body: OpenAiChatResponse = response.json().await?;
        let content = response_body
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| LlmError::DeserializationFailed("No content in response choices".to_string()))?;
            
        Ok(LlmResponse { content })
    }
} 