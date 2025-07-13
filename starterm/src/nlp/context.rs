//! Defines the contextual information provided to the NLP engine.

/// Provides context for NLP analysis.
pub struct NlpContext {
    /// The text being analyzed.
    pub text: String,
    /// The shell environment (e.g., "bash", "zsh"), if known.
    /// This helps differentiate between natural language and shell commands.
    pub shell_type: Option<String>,
} 