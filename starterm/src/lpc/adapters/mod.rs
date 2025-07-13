//! Contains adapters for different shell languages. Each adapter knows
//! how to parse its own syntax and convert it to/from the generic
//! `CommandIr`.

use crate::analysis::command_parser::CommandParser;
use crate::analysis::syntax_tree::SyntaxTree;
use crate::lpc::translator::{CommandIr, Result, TranslationResult};

// TODO: Import concrete adapter implementations.
// pub mod bash;
// pub mod fish;
// pub mod powershell;

/// The trait that all shell adapters must implement.
pub trait ShellAdapter {
    /// The name of the shell (e.g., "bash").
    fn name(&self) -> &'static str;

    /// The parser for this shell's grammar.
    fn parser(&self) -> &CommandParser;

    /// Parses a string into a shell-specific syntax tree.
    fn parse(&self, command: &str) -> Option<SyntaxTree> {
        self.parser().parse(command, None)
    }

    /// Converts a shell-specific `SyntaxTree` to the generic `CommandIr`.
    fn to_ir(&self, tree: &SyntaxTree) -> Result<CommandIr>;

    /// Converts a generic `CommandIr` to a command string for this shell.
    fn from_ir(&self, ir: &CommandIr) -> Result<TranslationResult>;
} 