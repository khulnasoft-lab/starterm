//! Implements modal input handling, similar to Vim. This allows for
//! different behaviors depending on the current mode (e.g., Normal, Insert).

use super::keysets::{Action, KeyCombination, KeysetManager};

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
    /// In `Insert` mode, it might return `None` to indicate the key
    /// should be passed through to the underlying shell.
    pub fn handle_key(&mut self, key: &KeyCombination) -> Option<Action> {
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