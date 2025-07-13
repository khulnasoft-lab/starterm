//! A language detection engine using the `whatlang` crate.

use whatlang::{detect, Lang};

/// Detects the most likely natural language of a given text snippet.
///
/// # Returns
///
/// An optional `Lang` enum representing the detected language (e.g., `Lang::Eng`).
/// Returns `None` if the text is too short or detection is unreliable.
pub fn detect_language(text: &str) -> Option<Lang> {
    detect(text).map(|info| info.lang())
} 