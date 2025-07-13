//! A cache for storing completion results to improve performance.

use super::providers::{Completion, CompletionContext};
use lru::LruCache;
use std::num::NonZeroUsize;

/// A simple LRU cache for completion suggestions.
/// The cache key is the input line up to the cursor.
pub struct CompletionCache {
    cache: LruCache<String, Vec<Completion>>,
}

impl CompletionCache {
    /// Creates a new cache with a specified capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(100).unwrap())),
        }
    }

    /// Gets completions for a given input from the cache.
    pub fn get(&mut self, context: &CompletionContext) -> Option<&Vec<Completion>> {
        let key = &context.line[..context.cursor_pos];
        self.cache.get(key)
    }

    /// Puts a set of completions into the cache.
    pub fn put(&mut self, context: &CompletionContext, completions: Vec<Completion>) {
        let key = context.line[..context.cursor_pos].to_string();
        self.cache.put(key, completions);
    }
}

impl Default for CompletionCache {
    fn default() -> Self {
        Self::new(500) // Default cache size
    }
}