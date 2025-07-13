//! Defines the configuration structures for workflows, which can be
//! loaded from files (e.g., YAML or TOML).

use crate::validation::ValidationRule;
use serde::{Deserialize, Serialize};

/// Represents a single choice in a multiple-choice question.
#[derive(Debug, Serialize, Deserialize)]
pub struct McqChoice {
    pub display_text: String,
    pub value: String,
}

/// Represents a request for user input via a multiple-choice question.
#[derive(Debug, Serialize, Deserialize)]
pub struct McqStep {
    pub id: String,
    pub prompt: String,
    pub choices: Vec<McqChoice>,
    #[serde(default)]
    pub validation_rules: Vec<ValidationRule>,
}

/// Represents a step in a workflow.
#[derive(Debug, Serialize, Deserialize)]
pub enum WorkflowStep {
    Execute(String), // A shell command to execute
    Ask(McqStep),    // An MCQ to present to the user
}

/// The top-level structure for a workflow configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
} 