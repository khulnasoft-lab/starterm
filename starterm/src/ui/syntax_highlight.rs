//! Generates highlighting information based on a `SyntaxTree`.

use crate::analysis::syntax_tree::SyntaxTree;

/// Represents a colored span of text.
#[derive(Debug, PartialEq)]
pub struct HighlightedSpan {
    pub start: usize,
    pub end: usize,
    /// A style identifier like "token.command", "token.string", etc.
    pub style_id: &'static str,
}

/// A highlighter that traverses a syntax tree to generate styles.
pub struct SyntaxHighlighter;

impl SyntaxHighlighter {
    /// Analyzes a `SyntaxTree` and returns a list of styled spans.
    pub fn highlight(tree: &SyntaxTree, source_code: &str) -> Vec<HighlightedSpan> {
        let mut highlights = Vec::new();
        let root = tree.root_node();
        let _cursor = root.walk();

        // TODO: Implement the tree traversal logic.
        // This typically involves a depth-first walk of the tree.
        // For each node, check its `kind()` (e.g., "command_name", "string_literal")
        // and map it to a style_id.
        // Pre-order traversal is often best for this.

        // Placeholder logic
        highlights.push(HighlightedSpan {
            start: 0,
            end: source_code.len(),
            style_id: "token.default",
        });

        highlights
    }
} 