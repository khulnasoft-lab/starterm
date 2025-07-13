//! The core completion engine that orchestrates providers and manages results.

use super::cache::CompletionCache;
use super::providers::{Completion, CompletionContext, CompletionProvider};

/// The main entry point for the autocompletion system.
pub struct CompletionEngine {
    providers: Vec<Box<dyn CompletionProvider + Send + Sync>>,
    cache: CompletionCache,
}

impl CompletionEngine {
    /// Creates a new completion engine.
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            cache: CompletionCache::default(),
        }
    }

    /// Registers a new completion provider.
    pub fn add_provider(&mut self, provider: Box<dyn CompletionProvider + Send + Sync>) {
        self.providers.push(provider);
    }

    /// Generates completions for the given context.
    pub fn get_completions(&mut self, context: &CompletionContext) -> Vec<Completion> {
        // Check cache first.
        if let Some(cached) = self.cache.get(context) {
            return cached.clone();
        }

        // Gather results from all providers.
        let mut all_completions = Vec::new();
        for provider in &self.providers {
            all_completions.extend(provider.provide(context));
        }

        // TODO: Implement intelligent ranking and filtering.
        // - Deduplicate results.
        // - Rank based on provider priority.
        // - Use ML models for contextual ranking (`candle-nn`).

        // Cache the final results.
        self.cache.put(context, all_completions.clone());

        all_completions
    }
}

impl Default for CompletionEngine {
    fn default() -> Self {
        // TODO: In a real implementation, load default providers here.
        // e.g., `self.add_provider(Box::new(FileProvider::new()));`
        Self::new()
    }
}