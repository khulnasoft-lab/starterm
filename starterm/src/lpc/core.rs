//! The central facade for the Language Processing Core.

use super::adapters::ShellAdapter;
use super::translator::{TranslationError, TranslationResult};

/// The main engine for LPC operations.
pub struct LpcEngine {
    // TODO: This should be a map of shell names to adapter instances.
    // e.g., `adapters: HashMap<String, Box<dyn ShellAdapter>>`
}

impl LpcEngine {
    pub fn new() -> Self {
        // TODO: Load all available shell adapters.
        Self {}
    }

    /// Translates a command from a source shell to a target shell.
    pub fn translate(
        &self,
        source_command: &str,
        source_shell: &dyn ShellAdapter,
        target_shell: &dyn ShellAdapter,
    ) -> Result<TranslationResult, TranslationError> {
        // 1. Parse the source command using the source shell's adapter.
        let ast = source_shell
            .parse(source_command)
            .ok_or(TranslationError::ParsingFailed)?;

        // 2. The source adapter converts the concrete syntax tree into a
        //    generic, intermediate representation (IR) of the command.
        let command_ir = source_shell.to_ir(&ast)?;

        // 3. The target adapter takes the IR and generates the
        //    equivalent command string in its own syntax.
        target_shell.from_ir(&command_ir)
    }
} 