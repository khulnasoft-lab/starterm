//! Logic for adapting the application's behavior based on detected language.

use whatlang::Lang;

/// An action to be taken based on the detected language.
pub enum NlpAction {
    /// Suggest translating a command or text.
    SuggestTranslation { from: Lang, to: Lang },
    /// Switch spell-check dictionary.
    SwitchDictionary(Lang),
    /// No action needed.
    None,
}

/// Determines what action to take based on detected language and app state.
pub fn determine_adaptation(detected_lang: Lang, app_default_lang: Lang) -> NlpAction {
    if detected_lang != app_default_lang {
        NlpAction::SuggestTranslation {
            from: detected_lang,
            to: app_default_lang,
        }
    } else {
        NlpAction::None
    }
} 