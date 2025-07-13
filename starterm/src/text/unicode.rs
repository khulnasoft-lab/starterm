//! Unicode-specific helper functions, primarily using the `unicode-segmentation`
//! crate to correctly handle grapheme clusters.

use unicode_segmentation::UnicodeSegmentation;

/// Counts the number of grapheme clusters in a string slice.
pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

/// Returns an iterator over the grapheme clusters of a string slice.
pub fn graphemes(s: &str) -> impl Iterator<Item = &str> {
    s.graphemes(true)
}