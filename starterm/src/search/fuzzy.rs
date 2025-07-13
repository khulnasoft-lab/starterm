//! Implements a fuzzy search algorithm for quickly finding matches
//! in terminal content, command history, or files.

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

/// A ranked search result from the fuzzy finder.
pub struct FuzzyMatch {
    pub text: String,
    pub score: i64,
    /// Indices of the matched characters in `text`.
    pub indices: Vec<usize>,
}

/// The core fuzzy matching engine.
pub struct FuzzySearcher {
    matcher: SkimMatcherV2,
}

impl FuzzySearcher {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
        }
    }

    /// Finds and ranks fuzzy matches for a pattern within a list of choices.
    pub fn find<'a, 'b>(
        &self,
        pattern: &'a str,
        choices: &'b [String],
    ) -> Vec<FuzzyMatch> {
        let mut results = choices
            .iter()
            .filter_map(|choice| {
                self.matcher.fuzzy_indices(choice, pattern).map(
                    |(score, indices)| FuzzyMatch {
                        text: choice.clone(),
                        score,
                        indices,
                    },
                )
            })
            .collect::<Vec<_>>();

        // Sort results by score, descending.
        results.sort_by(|a, b| b.score.cmp(&a.score));

        // TODO: Implement advanced ranking, including:
        // - Character proximity scoring
        // - Word boundary detection and scoring
        // - Recency bias for command history searches

        results
    }
}

impl Default for FuzzySearcher {
    fn default() -> Self {
        Self::new()
    }
}