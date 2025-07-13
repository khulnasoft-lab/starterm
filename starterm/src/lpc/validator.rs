//! Validation utilities for the LPC system.

use crate::lpc::translator::CommandIr;

/// Validates a command IR for correctness and completeness.
pub struct CommandValidator;

impl CommandValidator {
    /// Validates a command IR and returns any validation errors.
    pub fn validate(ir: &CommandIr) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Basic validation rules
        if ir.command_name.is_empty() {
            errors.push("Command name cannot be empty".to_string());
        }
        
        // TODO: Add more validation rules for arguments, assignments, etc.
        
        errors
    }
} 