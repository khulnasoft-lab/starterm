//! The LLM submodule for connecting to various language model providers.
//!
//! This module provides a generic `LlmClient` trait and concrete implementations
//! for different services like OpenAI, Anthropic, or local endpoints.

pub mod client;
pub mod openai;
// pub mod anthropic; // Future implementation
// pub mod local;     // Future implementation 