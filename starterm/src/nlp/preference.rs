//! Manages user language preferences and learned behavior.

use whatlang::Lang;
use std::collections::HashMap;

/// Stores user preferences and learned language patterns.
pub struct LanguagePreferences {
    // A map to store the frequency of detected languages to learn user patterns.
    detected_lang_counts: HashMap<Lang, u32>,
    // An explicitly set user preference.
    user_override: Option<Lang>,
}

impl LanguagePreferences {
    pub fn new() -> Self {
        Self {
            detected_lang_counts: HashMap::new(),
            user_override: None,
        }
    }

    /// Records a language detection result to learn from it.
    pub fn record_detection(&mut self, lang: Lang) {
        *self.detected_lang_counts.entry(lang).or_insert(0) += 1;
    }

    /// Gets the most likely language based on learned preferences.
    pub fn get_preferred_language(&self) -> Option<Lang> {
        self.user_override.or_else(|| {
            self.detected_lang_counts
                .iter()
                .max_by_key(|&(_, count)| count)
                .map(|(lang, _)| *lang)
        })
    }
} 