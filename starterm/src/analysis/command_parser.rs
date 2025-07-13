//! The core command parser, using the `tree-sitter` library.

use super::syntax_tree::SyntaxTree;
use std::sync::Mutex;
use tree_sitter::{Parser, Language};

/// A reusable parser for shell commands.
///
/// It holds a `tree-sitter` parser and the corresponding language grammar.
/// It is wrapped in a Mutex to allow safe use across threads, as `Parser`
/// is not `Sync`.
pub struct CommandParser {
    parser: Mutex<Parser>,
}

impl CommandParser {
    /// Creates a new parser for a specific shell language (e.g., Bash).
    pub fn new(language: Language) -> Result<Self, String> {
        let mut parser = Parser::new();
        parser
            .set_language(language)
            .map_err(|e| e.to_string())?;
        Ok(Self {
            parser: Mutex::new(parser),
        })
    }

    /// Parses a source code string into a `SyntaxTree`.
    ///
    /// The `old_tree` argument can be used for incremental parsing,
    /// which significantly improves performance for repeated edits.
    pub fn parse(&self, source_code: &str, old_tree: Option<&SyntaxTree>) -> Option<SyntaxTree> {
        let mut parser = self.parser.lock().unwrap();
        let tree = parser.parse(source_code, old_tree.map(|st| &st.tree));
        tree.map(SyntaxTree::new)
    }
}

// TODO: In the application's setup, you would initialize this once:
// `let bash_parser = CommandParser::new(tree_sitter_bash::language()).unwrap();` 