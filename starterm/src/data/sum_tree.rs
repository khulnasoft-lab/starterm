//! The core Sum Tree data structure.
//!
//! A Sum Tree is a variant of a B-tree where each internal node stores the
//! sum of a metric over all its children. For text, this allows for O(log n)
//! lookup of a line or character by its index.

use super::indexing::TextMetric;

/// A node in the Sum Tree.
enum Node {
    Leaf(String),
    Internal(Vec<Box<Node>>),
}

/// The Sum Tree, optimized for text operations.
pub struct SumTree {
    root: Box<Node>,
    // The metric being tracked by the tree (e.g., line count).
    metric: Box<dyn TextMetric<Value = u64>>,
}

impl SumTree {
    /// Creates a new Sum Tree with a specific metric.
    pub fn new(metric: Box<dyn TextMetric<Value = u64>>) -> Self {
        Self {
            root: Box::new(Node::Leaf(String::new())),
            metric,
        }
    }

    // TODO: Implement the complex tree operations.
    // pub fn insert(&mut self, offset: usize, text: &str) {}
    // pub fn delete(&mut self, range: std::ops::Range<usize>) {}

    /// Retrieves a chunk of text and its metadata at a specific metric offset.
    pub fn get_by_metric(&self, _offset: u64) -> Option<String> {
        // TODO: Implement O(log n) traversal using the summed metrics in internal nodes.
        None
    }
} 