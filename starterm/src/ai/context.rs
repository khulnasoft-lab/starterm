//! Defines the rich, structured context provided to the AI systems.

/// A snapshot of the terminal's state and history for AI analysis.
/// This gathers information from various other modules.
pub struct AiContext<'a> {
    /// The current command line input.
    pub current_input: &'a str,
    /// Recent lines from the terminal's scrollback buffer.
    /// This would be fetched from the `SumTree` data structure.
    pub scrollback: Vec<&'a str>,
    /// The current working directory.
    pub cwd: &'a str,
    /// The detected shell type (e.g., "bash", "zsh").
    /// From the LPC or NLP modules.
    pub shell_type: &'a str,
    /// The exit code of the last command.
    pub last_exit_code: Option<i32>,
    // TODO: Add more context, such as git repository status,
    // open network connections, etc.
} 