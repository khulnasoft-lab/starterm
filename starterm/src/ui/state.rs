//! Manages the state of various UI components, especially for AI interactions.

use crate::ai::suggestions::Suggestion;

/// Represents the state of the suggestions overlay.
#[derive(Debug, Default)]
pub struct SuggestionsState {
    /// The list of suggestions currently being displayed.
    pub suggestions: Vec<Suggestion>,
    /// The index of the currently selected suggestion.
    pub selected_index: usize,
}

/// Represents the overall state of the UI's interactive components.
#[derive(Debug, Default)]
pub enum InteractiveState {
    /// Default state, no special UI is active.
    #[default]
    Default,
    /// The suggestions overlay is visible and interactive.
    Suggestions(SuggestionsState),
    /// The user is typing a prompt in the dedicated Agent Mode input.
    AgentInput,
}

/// The main struct holding all UI state.
#[derive(Debug, Default)]
pub struct UiState {
    pub interactive_state: InteractiveState,
}

impl UiState {
    /// Shows the suggestions overlay with a given list of suggestions.
    pub fn show_suggestions(&mut self, suggestions: Vec<Suggestion>) {
        if !suggestions.is_empty() {
            self.interactive_state = InteractiveState::Suggestions(SuggestionsState {
                suggestions,
                selected_index: 0,
            });
        } else {
            self.dismiss_interactive();
        }
    }

    /// Hides any interactive UI element and returns to the default state.
    pub fn dismiss_interactive(&mut self) {
        self.interactive_state = InteractiveState::Default;
    }
} 