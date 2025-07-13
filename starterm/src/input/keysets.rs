//! Defines the keybinding system, allowing users to map key combinations
//! to specific actions within the terminal.

use std::collections::HashMap;
use crate::ai::suggestions::SuggestionAction; // Import suggestion action

// TODO: This should be an exhaustive enum of all possible actions in Starterm.
#[derive(Debug, Clone, PartialEq)] // Removed Eq, Hash as SuggestionAction is not hashable
pub enum Action {
    ScrollUp,
    ScrollDown,
    Copy,
    Paste,
    EnterSearchMode,
    Quit,
    /// A new action to apply an AI suggestion.
    ApplySuggestion(SuggestionAction),
}

// TODO: This should represent a key combination, including modifiers.
// It will likely be built from `winit` event types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombination {
    pub key: String,
    pub mods: String, // e.g., "Ctrl+Shift"
}

/// A set of keybindings for a specific mode.
pub type Keymap = HashMap<KeyCombination, Action>;

/// Manages all available keysets and the currently active one.
pub struct KeysetManager {
    keysets: HashMap<String, Keymap>,
    active_keyset: String,
}

impl KeysetManager {
    pub fn new() -> Self {
        // TODO: Load default and user-defined keysets from config files.
        Self {
            keysets: HashMap::new(),
            active_keyset: "default".to_string(),
        }
    }

    /// Resolves a key combination to an action based on the active keyset.
    pub fn resolve(&self, key_comb: &KeyCombination) -> Option<&Action> {
        self.keysets
            .get(&self.active_keyset)
            .and_then(|keymap| keymap.get(key_comb))
    }

    /// Sets the active keymap.
    pub fn set_active_keyset(&mut self, name: &str) {
        // TODO: Add error handling for non-existent keysets.
        self.active_keyset = name.to_string();
    }
}

impl Default for KeysetManager {
    fn default() -> Self {
        Self::new()
    }
}