//! Implements modal input handling, similar to Vim. This allows for
//! different behaviors depending on the current mode (e.g., Normal, Insert).

use super::keysets::{Action, KeyCombination, KeysetManager};
use crate::ui::state::{InteractiveState, UiState}; // Import UI state

/// Defines the possible input modes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    /// Standard terminal input mode. Keystrokes are sent to the PTY.
    Insert,
    /// Command mode for navigation and application control.
    Normal,
    /// Visual selection mode.
    Visual,
    /// Search mode.
    Search,
}

/// Manages the current input mode and dispatches actions accordingly.
pub struct ModalHandler {
    mode: InputMode,
    keyset_manager: KeysetManager,
}

impl ModalHandler {
    pub fn new(keyset_manager: KeysetManager) -> Self {
        Self {
            mode: InputMode::Insert, // Default to insert mode for a terminal
            keyset_manager,
        }
    }

    /// Processes a key combination and returns an optional action.
    /// It now takes the UI state to decide how to handle input.
    pub fn handle_key(&mut self, key: &KeyCombination, ui_state: &mut UiState) -> Option<Action> {
        // First, check if an interactive UI component should capture the input.
        if let InteractiveState::Suggestions(state) = &mut ui_state.interactive_state {
            match key.key.as_str() {
                "Tab" | "ArrowDown" => {
                    state.selected_index = (state.selected_index + 1) % state.suggestions.len();
                    return None; // Input handled by UI, no further action needed.
                }
                "ArrowUp" => {
                    state.selected_index = (state.selected_index + state.suggestions.len() - 1) % state.suggestions.len();
                    return None;
                }
                "Enter" => {
                    if let Some(suggestion) = state.suggestions.get(state.selected_index) {
                        // Return a new action to apply the suggestion.
                        return Some(Action::ApplySuggestion(suggestion.action.clone()));
                    }
                }
                "Escape" => {
                    ui_state.dismiss_interactive();
                    return None;
                }
                _ => {} // Other keys are ignored when suggestions are active.
            }
        }

        // If no interactive UI handled the key, proceed with normal logic.
        match self.mode {
            InputMode::Insert => {
                // TODO: Check for an "escape" keybinding to switch to Normal mode.
                // Otherwise, pass through.
                None
            }
            InputMode::Normal | InputMode::Visual | InputMode::Search => {
                // In other modes, resolve the key against the current keyset.
                self.keyset_manager.resolve(key).cloned()
            }
        }
    }

    /// Changes the current input mode.
    pub fn set_mode(&mut self, mode: InputMode) {
        self.mode = mode;
        // TODO: Potentially switch the active keyset in the KeysetManager.
        // For example, `self.keyset_manager.set_active_keyset("normal-mode");`
    }

    pub fn current_mode(&self) -> &InputMode {
        &self.mode
    }
}