//! Defines the generic trait for LLM clients and shared data structures.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// A generic representation of a message in a chat-based LLM prompt.
#[derive(Debug, Serialize, Clone)]
pub struct ChatMessage {
    pub role: String, // e.g., "system", "user", "assistant"
    pub content: String,
}

/// A generic request structure to be sent to an LLM.
#[derive(Debug, Serialize)]
pub struct LlmRequest {
    pub messages: Vec<ChatMessage>,
    pub model: String,
    // Add other common parameters like temperature, max_tokens, etc.
}

/// A generic response structure from an LLM.
#[derive(Debug, Deserialize)]
pub struct LlmResponse {
    pub content: String,
    // Add other fields like token usage, finish reason, etc.
}

/// Custom error types for LLM client operations.
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("Network request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("API returned an error: {status}: {body}")]
    ApiError { status: u16, body: String },
    #[error("Failed to deserialize response: {0}")]
    DeserializationFailed(String),
}

/// The core asynchronous trait that all LLM clients must implement.
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Executes a prompt and returns the LLM's response.
    async fn execute_chat(&self, request: LlmRequest) -> Result<LlmResponse, LlmError>;
} 