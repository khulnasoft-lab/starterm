//! Defines the `CompletionProvider` trait and common completion types.
//! Any struct that implements this trait can be used as a source for completions.

/// The context for a completion request.
pub struct CompletionContext<'a> {
    /// The full text on the current line.
    pub line: &'a str,
    /// The byte offset of the cursor within the line.
    pub cursor_pos: usize,
}

/// A single completion suggestion.
#[derive(Debug, Clone)]
pub struct Completion {
    /// The text to be inserted.
    pub value: String,
    /// Optional display text for the UI (if different from `value`).
    pub display: Option<String>,
    /// Optional description for the completion.
    pub description: Option<String>,
}

/// The trait that all completion providers must implement.
pub trait CompletionProvider {
    /// The name of the provider (for debugging and configuration).
    fn name(&self) -> &str;

    /// Generates a list of completions based on the given context.
    fn provide(&self, context: &CompletionContext) -> Vec<Completion>;
}

// TODO: Implement concrete providers in this module, e.g.,
// pub mod file_provider;
// pub mod history_provider;
// pub mod git_provider;