//! The central AI coordinator.
//!
//! This acts as the main entry point for all AI-related functionality,
//! managing different sub-systems like the agent, suggestion engine,
//! and learning components.

use super::agent::{Agent, Agentic};
use super::context::AiContext;
use super::reasoning::ReasoningEngine;
use super::suggestions::{Suggestion, SuggestionEngine};
use super::workflow_gen::GeneratedWorkflow;
use crate::ai::llm::client::LlmClient;
use std::sync::Arc;

/// The unified AI Core for the terminal.
pub struct AiCore {
    agent: Agent,
    suggestion_engine: SuggestionEngine,
    reasoning_engine: Arc<ReasoningEngine>, // Added
}

impl AiCore {
    /// Creates a new `AiCore` with a given LLM client.
    pub fn new(llm_client: Arc<dyn LlmClient>) -> Self {
        let reasoning_engine = Arc::new(ReasoningEngine::new());
        Self {
            agent: Agent::new(llm_client),
            suggestion_engine: SuggestionEngine::new(reasoning_engine.clone()),
            reasoning_engine,
        }
    }

    /// The main "tick" function for the AI, to be called periodically or on events.
    /// It generates proactive suggestions based on the current context.
    pub fn get_suggestions(&self, context: &AiContext) -> Vec<Suggestion> {
        self.suggestion_engine.generate(context)
    }

    /// Evaluates a natural language prompt using the AI Agent to generate a workflow.
    pub async fn evaluate_prompt(&self, prompt: &str, context: &AiContext<'_>) -> Result<GeneratedWorkflow, String> {
        self.agent.evaluate(prompt, context).await
    }
} 