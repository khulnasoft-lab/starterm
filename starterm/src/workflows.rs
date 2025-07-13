//! Workflows integration for Starterm.
//!
//! This module exposes the workflows API and provides functionality
//! for managing and executing workflows within the terminal.

use std::collections::HashMap;
use std::fmt;

pub use starterm_workflows as workflows_crate;
pub use workflows_crate::{workflows, Workflow};

/// Manager for workflow execution and state
#[derive(Debug, Clone)]
pub struct WorkflowManager {
    workflows: Vec<Workflow>,
    current_selection: Option<usize>,
    search_filter: String,
    argument_values: HashMap<String, String>,
}

impl WorkflowManager {
    /// Create a new workflow manager
    pub fn new() -> Self {
        Self {
            workflows: workflows(),
            current_selection: None,
            search_filter: String::new(),
            argument_values: HashMap::new(),
        }
    }

    /// Get all workflows
    pub fn workflows(&self) -> &[Workflow] {
        &self.workflows
    }

    /// Get filtered workflows based on search criteria
    pub fn filtered_workflows(&self) -> Vec<(usize, &Workflow)> {
        if self.search_filter.is_empty() {
            return self.workflows.iter().enumerate().collect();
        }

        let filter_lower = self.search_filter.to_lowercase();
        self.workflows
            .iter()
            .enumerate()
            .filter(|(_, workflow)| {
                workflow.name().to_lowercase().contains(&filter_lower)
                    || workflow.tags().iter().any(|tag| tag.to_lowercase().contains(&filter_lower))
                    || workflow.description()
                        .as_ref()
                        .map_or(false, |desc| desc.to_lowercase().contains(&filter_lower))
            })
            .collect()
    }

    /// Get the currently selected workflow
    pub fn selected_workflow(&self) -> Option<&Workflow> {
        self.current_selection.and_then(|idx| self.workflows.get(idx))
    }

    /// Set the current selection
    pub fn set_selection(&mut self, index: Option<usize>) {
        self.current_selection = index;
    }

    /// Get the current selection index
    pub fn selection(&self) -> Option<usize> {
        self.current_selection
    }

    /// Move selection up
    pub fn move_selection_up(&mut self) {
        let filtered = self.filtered_workflows();
        if filtered.is_empty() {
            return;
        }

        match self.current_selection {
            Some(current) => {
                if let Some(pos) = filtered.iter().position(|(idx, _)| *idx == current) {
                    let new_pos = if pos > 0 { pos - 1 } else { filtered.len() - 1 };
                    self.current_selection = Some(filtered[new_pos].0);
                }
            }
            None => {
                self.current_selection = Some(filtered[0].0);
            }
        }
    }

    /// Move selection down
    pub fn move_selection_down(&mut self) {
        let filtered = self.filtered_workflows();
        if filtered.is_empty() {
            return;
        }

        match self.current_selection {
            Some(current) => {
                if let Some(pos) = filtered.iter().position(|(idx, _)| *idx == current) {
                    let new_pos = if pos < filtered.len() - 1 { pos + 1 } else { 0 };
                    self.current_selection = Some(filtered[new_pos].0);
                }
            }
            None => {
                self.current_selection = Some(filtered[0].0);
            }
        }
    }

    /// Set the search filter
    pub fn set_search_filter(&mut self, filter: String) {
        self.search_filter = filter;
        // Reset selection when filter changes
        let filtered = self.filtered_workflows();
        if !filtered.is_empty() {
            self.current_selection = Some(filtered[0].0);
        } else {
            self.current_selection = None;
        }
    }

    /// Get the current search filter
    pub fn search_filter(&self) -> &str {
        &self.search_filter
    }

    /// Set argument value for a workflow
    pub fn set_argument_value(&mut self, arg_name: String, value: String) {
        self.argument_values.insert(arg_name, value);
    }

    /// Get argument value for a workflow
    pub fn get_argument_value(&self, arg_name: &str) -> Option<&String> {
        self.argument_values.get(arg_name)
    }

    /// Clear all argument values
    pub fn clear_argument_values(&mut self) {
        self.argument_values.clear();
    }

    /// Generate the command for the selected workflow with current argument values
    pub fn generate_command(&self) -> Result<String, WorkflowError> {
        let workflow = self.selected_workflow()
            .ok_or(WorkflowError::NoWorkflowSelected)?;

        let mut command = workflow.command().to_string();

        // Replace argument placeholders with values
        for arg in workflow.arguments() {
            let placeholder = format!("{{{{{}}}}}", arg.name());
            let value = self.get_argument_value(arg.name())
                .cloned()
                .or_else(|| arg.default_value().clone())
                .unwrap_or_default();
            
            command = command.replace(&placeholder, &value);
        }

        Ok(command)
    }

    /// Check if all required arguments have values
    pub fn are_arguments_complete(&self) -> bool {
        if let Some(workflow) = self.selected_workflow() {
            for arg in workflow.arguments() {
                if self.get_argument_value(arg.name()).is_none() 
                    && arg.default_value().is_none() {
                    return false;
                }
            }
        }
        true
    }
}

impl Default for WorkflowManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Error types for workflow operations
#[derive(Debug, Clone)]
pub enum WorkflowError {
    /// No workflow is currently selected
    NoWorkflowSelected,
    /// Missing required argument
    MissingArgument(String),
    /// Invalid argument value
    InvalidArgument(String, String),
}

impl fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowError::NoWorkflowSelected => write!(f, "No workflow is selected"),
            WorkflowError::MissingArgument(arg) => write!(f, "Missing required argument: {}", arg),
            WorkflowError::InvalidArgument(arg, reason) => {
                write!(f, "Invalid argument '{}': {}", arg, reason)
            }
        }
    }
}

impl std::error::Error for WorkflowError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_manager_creation() {
        let manager = WorkflowManager::new();
        assert!(!manager.workflows().is_empty());
    }

    #[test]
    fn test_workflow_selection() {
        let mut manager = WorkflowManager::new();
        assert_eq!(manager.selection(), None);
        
        manager.set_selection(Some(0));
        assert_eq!(manager.selection(), Some(0));
        
        manager.set_selection(None);
        assert_eq!(manager.selection(), None);
    }

    #[test]
    fn test_workflow_navigation() {
        let mut manager = WorkflowManager::new();
        
        // Test moving down when no selection
        manager.move_selection_down();
        assert_eq!(manager.selection(), Some(0));
        
        // Test moving up from first item (should wrap to last)
        let workflow_count = manager.workflows().len();
        manager.move_selection_up();
        assert_eq!(manager.selection(), Some(workflow_count - 1));
        
        // Test moving down from last item (should wrap to first)
        manager.move_selection_down();
        assert_eq!(manager.selection(), Some(0));
    }

    #[test]
    fn test_search_filter() {
        let mut manager = WorkflowManager::new();
        
        // Test empty filter
        let all_workflows = manager.filtered_workflows();
        assert_eq!(all_workflows.len(), manager.workflows().len());
        
        // Test filter that matches something
        manager.set_search_filter("git".to_string());
        let filtered = manager.filtered_workflows();
        assert!(!filtered.is_empty());
        assert!(filtered.iter().all(|(_, w)| {
            w.name().to_lowercase().contains("git") ||
            w.tags().iter().any(|tag| tag.to_lowercase().contains("git"))
        }));
    }

    #[test]
    fn test_argument_handling() {
        let mut manager = WorkflowManager::new();
        
        // Set an argument value
        manager.set_argument_value("message".to_string(), "test commit".to_string());
        assert_eq!(manager.get_argument_value("message"), Some(&"test commit".to_string()));
        
        // Clear arguments
        manager.clear_argument_values();
        assert_eq!(manager.get_argument_value("message"), None);
    }
}
