//! The intelligent suggestion engine.

use super::context::AiContext;

/// Represents a single proactive suggestion from the AI.
pub struct Suggestion {
    pub text: String,
    pub action: SuggestionAction,
}

/// An action associated with a suggestion.
pub enum SuggestionAction {
    /// Insert text into the command line.
    InsertText(String),
    /// Run a full command.
    RunCommand(String),
    /// Start a workflow.
    RunWorkflow(String), // The name of the workflow
}

/// The engine responsible for generating suggestions.
pub struct SuggestionEngine;

impl SuggestionEngine {
    pub fn new() -> Self {
        Self
    }

    /// Generates a list of suggestions based on the current context.
    pub fn generate(&self, context: &AiContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // TODO: Implement rule-based and ML-based suggestion logic.
        // Example: If the last command failed, suggest a fix.
        if context.last_exit_code == Some(127) && context.current_input.is_empty() {
             suggestions.push(Suggestion {
                text: "Last command not found. Search for package to install it?".to_string(),
                action: SuggestionAction::InsertText("apt search <command>".to_string()),
             });
        }
        
        suggestions
    }
} 