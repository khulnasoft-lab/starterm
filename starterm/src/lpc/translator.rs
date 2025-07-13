//! Defines the traits and data structures for the translation process.

/// A generic, intermediate representation of a shell command.
/// This abstracts away the specific syntax of any single shell.
#[derive(Debug)]
pub struct CommandIr {
    pub command_name: String,
    pub arguments: Vec<String>,
    pub assignments: Vec<(String, String)>,
    // TODO: Add more complex structures like pipes, redirections, etc.
}

/// A successful translation result.
pub struct TranslationResult {
    pub translated_command: String,
}

/// An error that occurred during translation.
#[derive(Debug)]
pub enum TranslationError {
    ParsingFailed,
    IrConversionFailed(String),
    UnsupportedFeature(String),
}

/// The result type for translation operations.
pub type Result<T, E = TranslationError> = std::result::Result<T, E>; 