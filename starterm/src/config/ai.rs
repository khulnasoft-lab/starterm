//! Configuration for AI-related features.

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct AiConfig {
    /// Configuration for the OpenAI provider.
    #[serde(default)]
    pub openai: OpenAiConfig,
    // TODO: Add other providers like `anthropic` here.
}

#[derive(Debug, Deserialize, Default)]
pub struct OpenAiConfig {
    /// The API key for OpenAI services.
    pub api_key: Option<String>,
    /// The API URL, can be overridden for custom endpoints.
    pub api_url: Option<String>,
} 