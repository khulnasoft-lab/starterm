//! The intelligent suggestion engine.

use super::context::AiContext;
use super::reasoning::{Analysis, ReasoningEngine};
use std::sync::Arc;

/// Represents a single proactive suggestion from the AI.
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub text: String,
    pub action: SuggestionAction,
}

/// An action associated with a suggestion.
#[derive(Debug, Clone, PartialEq)]
pub enum SuggestionAction {
    /// Insert text into the command line.
    InsertText(String),
    /// Run a full command.
    RunCommand(String),
    /// Start a workflow.
    RunWorkflow(String), // The name of the workflow
}

/// The engine responsible for generating suggestions.
pub struct SuggestionEngine {
    reasoning_engine: Arc<ReasoningEngine>,
}

impl SuggestionEngine {
    pub fn new(reasoning_engine: Arc<ReasoningEngine>) -> Self {
        Self { reasoning_engine }
    }

    /// Generates a list of suggestions based on the current context.
    pub fn generate(&self, context: &AiContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // 1. Get high-level analysis from the reasoning engine.
        let analysis = self.reasoning_engine.analyze(context);

        // 2. Apply rule-based suggestions based on the analysis.
        if let Some(Analysis::CommandFailed(failure)) = analysis {
            match failure.reason.as_str() {
                "Command Not Found" => {
                    suggestions.push(Suggestion {
                        text: format!("Fix: Install '{}'?", failure.command),
                        action: SuggestionAction::InsertText(format!("sudo apt install {} # or brew, dnf, etc.", failure.command)),
                    });
                }
                "Permission Denied" => {
                     suggestions.push(Suggestion {
                        text: "Fix: Rerun with 'sudo'?".to_string(),
                        action: SuggestionAction::InsertText(format!("sudo {}", failure.full_input)),
                    });
                }
                _ => {}
            }
        }

        // Example: 'git' command suggestions
        if context.current_input.starts_with("git ") {
            if context.current_input.contains("status") {
                suggestions.push(Suggestion {
                    text: "Next: `git add .`".to_string(),
                    action: SuggestionAction::InsertText("git add .".to_string()),
                });
            }
        }

        // TODO: Add a hook for LLM-based suggestions for complex cases.
        // `if suggestions.is_empty() { suggestions.extend(self.get_llm_suggestions(context)); }`

        suggestions
    }
} 