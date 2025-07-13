//! The reasoning engine for analyzing terminal context.
//!
//! This engine processes raw context and produces a higher-level,
//! structured `Analysis` enum, which other AI services can then use.

use super::context::AiContext;

/// Represents a high-level analysis of the current terminal state.
#[derive(Debug)]
pub enum Analysis<'a> {
    CommandFailed(CommandFailure<'a>),
    InGitRepository,
    // Add other states like InDockerContainer, SshSessionActive, etc.
    Idle,
}

/// Details about a failed command.
#[derive(Debug)]
pub struct CommandFailure<'a> {
    /// The specific command that failed (e.g., "gitt").
    pub command: &'a str,
    /// The full input line.
    pub full_input: &'a str,
    /// The reason for failure.
    pub reason: String,
}

/// The engine responsible for analyzing context.
pub struct ReasoningEngine;

impl ReasoningEngine {
    pub fn new() -> Self {
        Self
    }

    /// Analyzes the context and returns a structured `Analysis`.
    pub fn analyze<'a>(&self, context: &'a AiContext) -> Option<Analysis<'a>> {
        // Analyze the last command's exit code.
        if let Some(code) = context.last_exit_code {
            if code != 0 {
                let last_input = context.scrollback.last().unwrap_or(&"");
                let command = last_input.split_whitespace().next().unwrap_or("");
                
                // This is a simplified analysis. A real implementation would parse stderr.
                let reason = match code {
                    127 => "Command Not Found".to_string(),
                    126 | 1 => "Permission Denied".to_string(), // Common for permission issues
                    _ => format!("Exited with code {}", code),
                };

                return Some(Analysis::CommandFailed(CommandFailure {
                    command,
                    full_input: last_input,
                    reason,
                }));
            }
        }

        // TODO: Add more reasoning logic, e.g., check for `.git` directory.
        // if context.cwd contains a .git folder -> return Some(Analysis::InGitRepository)
        
        None
    }
}

impl Default for ReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
} 