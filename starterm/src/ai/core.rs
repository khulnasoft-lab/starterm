//! The central AI coordinator.
//!
//! This acts as the main entry point for all AI-related functionality,
//! managing different sub-systems like the agent, suggestion engine,
//! and learning components.

use super::agent::{Agent, Agentic};
use super::context::AiContext;
use super::suggestions::{Suggestion, SuggestionEngine};

/// The unified AI Core for the terminal.
pub struct AiCore {
    agent: Agent,
    suggestion_engine: SuggestionEngine,
    // TODO: Add other components like the learning and reasoning engines.
}

impl AiCore {
    /// Creates a new `AiCore`.
    pub fn new() -> Self {
        Self {
            agent: Agent::new(),
            suggestion_engine: SuggestionEngine::new(),
        }
    }

    /// The main "tick" function for the AI, to be called periodically or on events.
    /// It generates proactive suggestions based on the current context.
    pub fn get_suggestions(&self, context: &AiContext) -> Vec<Suggestion> {
        self.suggestion_engine.generate(context)
    }

    /// Evaluates a natural language prompt using the AI Agent to generate a workflow.
    pub async fn evaluate_prompt(&self, prompt: &str, context: &AiContext<'_>) -> Result<super::workflow_gen::GeneratedWorkflow, String> {
        self.agent.evaluate(prompt, context).await
    }
}

impl Default for AiCore {
    fn default() -> Self {
        Self::new()
    }
} 