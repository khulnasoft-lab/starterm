//! Defines the wrapper around a `tree-sitter` syntax tree, providing
//! convenient methods for querying and traversing the command structure.

use tree_sitter::{Node, Tree};

/// A wrapper for a `tree-sitter` Tree, providing an ergonomic API
/// for inspecting a parsed shell command.
pub struct SyntaxTree {
    pub tree: Tree,
}

impl SyntaxTree {
    pub fn new(tree: Tree) -> Self {
        Self { tree }
    }

    /// Returns the root node of the syntax tree.
    pub fn root_node(&self) -> Node<'_> {
        self.tree.root_node()
    }

    // TODO: Add methods for common queries, for example:
    // - `find_command_name(&self) -> Option<Node>`
    // - `find_all_arguments(&self) -> Vec<Node>`
    // - `node_at_position(&self, byte_pos: usize) -> Option<Node>`
    // - `get_node_text(&self, node: Node, source: &str) -> &str`
} 