//! The AI Agent, responsible for interpreting natural language and
//! performing complex, multi-step tasks.

use super::context::AiContext;
use super::workflow_gen::{generate_workflow_from_prompt, GeneratedWorkflow};
use crate::ai::llm::client::LlmClient;
use async_trait::async_trait;
use std::sync::Arc;

/// The trait for an AI agent capable of evaluating prompts.
#[async_trait]
pub trait Agentic {
    async fn evaluate(&self, prompt: &str, context: &AiContext) -> Result<GeneratedWorkflow, String>;
}

/// The primary implementation of the Starterm Agent.
pub struct Agent {
    llm_client: Arc<dyn LlmClient>,
}

impl Agent {
    pub fn new(llm_client: Arc<dyn LlmClient>) -> Self {
        Self { llm_client }
    }
}

#[async_trait]
impl Agentic for Agent {
    /// Takes a natural language prompt, analyzes it within the given context,
    /// and generates a structured, executable workflow.
    async fn evaluate(&self, prompt: &str, context: &AiContext) -> Result<GeneratedWorkflow, String> {
        // Now uses the client stored in `self`.
        generate_workflow_from_prompt(prompt, context, self.llm_client.clone()).await
    }
} 