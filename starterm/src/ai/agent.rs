//! The AI Agent, responsible for interpreting natural language and
//! performing complex, multi-step tasks.

use super::context::AiContext;
use super::workflow_gen::{generate_workflow_from_prompt, GeneratedWorkflow};
use async_trait::async_trait;

/// The trait for an AI agent capable of evaluating prompts.
#[async_trait]
pub trait Agentic {
    async fn evaluate(&self, prompt: &str, context: &AiContext) -> Result<GeneratedWorkflow, String>;
}

/// The primary implementation of the Starterm Agent.
pub struct Agent;

impl Agent {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Agentic for Agent {
    /// Takes a natural language prompt, analyzes it within the given context,
    /// and generates a structured, executable workflow.
    async fn evaluate(&self, prompt: &str, context: &AiContext) -> Result<GeneratedWorkflow, String> {
        // This is the core of "Agent Mode Eval". It calls the workflow generator.
        generate_workflow_from_prompt(prompt, context).await
    }
} 