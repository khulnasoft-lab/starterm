//! Tests for the AI UI integration components.

#[cfg(test)]
mod tests {
    use crate::ai::suggestions::{Suggestion, SuggestionAction};
    use crate::ui::state::{UiState, InteractiveState, SuggestionsState};
    use crate::ui::suggestions_view::SuggestionsView;
    use crate::input::keysets::{Action, KeyCombination};
    use crate::input::modal::ModalHandler;

    #[test]
    fn test_ui_state_suggestions() {
        let mut ui_state = UiState::default();
        
        // Test initial state
        assert!(matches!(ui_state.interactive_state, InteractiveState::Default));
        
        // Test showing suggestions
        let suggestions = vec![
            Suggestion {
                text: "ls -la".to_string(),
                action: SuggestionAction::RunCommand("ls -la".to_string()),
            },
            Suggestion {
                text: "pwd".to_string(),
                action: SuggestionAction::RunCommand("pwd".to_string()),
            },
        ];
        
        ui_state.show_suggestions(suggestions.clone());
        
        match &ui_state.interactive_state {
            InteractiveState::Suggestions(state) => {
                assert_eq!(state.suggestions.len(), 2);
                assert_eq!(state.selected_index, 0);
                assert_eq!(state.suggestions[0].text, "ls -la");
                assert_eq!(state.suggestions[1].text, "pwd");
            }
            _ => panic!("Expected Suggestions state"),
        }
        
        // Test dismissing suggestions
        ui_state.dismiss_interactive();
        assert!(matches!(ui_state.interactive_state, InteractiveState::Default));
    }

    #[test]
    fn test_suggestions_view_rendering() {
        let suggestions = vec![
            Suggestion {
                text: "ls -la".to_string(),
                action: SuggestionAction::RunCommand("ls -la".to_string()),
            },
            Suggestion {
                text: "pwd".to_string(),
                action: SuggestionAction::RunCommand("pwd".to_string()),
            },
        ];
        
        let state = SuggestionsState {
            suggestions,
            selected_index: 1, // Select the second suggestion
        };
        
        let styled_lines = SuggestionsView::render(&state);
        
        // Should have header + 2 suggestions = 3 lines
        assert_eq!(styled_lines.len(), 3);
        assert_eq!(styled_lines[0].0, "AI Suggestions:");
        assert_eq!(styled_lines[1].0, "  ls -la"); // Not selected
        assert_eq!(styled_lines[2].0, "> pwd");    // Selected
    }

    #[test]
    fn test_modal_handler_with_ui_state() {
        let keyset_manager = crate::input::keysets::KeysetManager::new();
        let mut modal_handler = ModalHandler::new(keyset_manager);
        let mut ui_state = UiState::default();
        
        // Set up suggestions
        let suggestions = vec![
            Suggestion {
                text: "ls -la".to_string(),
                action: SuggestionAction::RunCommand("ls -la".to_string()),
            },
            Suggestion {
                text: "pwd".to_string(),
                action: SuggestionAction::RunCommand("pwd".to_string()),
            },
        ];
        ui_state.show_suggestions(suggestions);
        
        // Test arrow down navigation
        let down_key = KeyCombination {
            key: "ArrowDown".to_string(),
            mods: "".to_string(),
        };
        
        let action = modal_handler.handle_key(&down_key, &mut ui_state);
        assert!(action.is_none()); // Should be handled by UI, no action returned
        
        // Check that selection moved to second item
        if let InteractiveState::Suggestions(state) = &ui_state.interactive_state {
            assert_eq!(state.selected_index, 1);
        } else {
            panic!("Expected Suggestions state");
        }
        
        // Test enter to apply suggestion
        let enter_key = KeyCombination {
            key: "Enter".to_string(),
            mods: "".to_string(),
        };
        
        let action = modal_handler.handle_key(&enter_key, &mut ui_state);
        assert!(action.is_some());
        
        if let Some(Action::ApplySuggestion(SuggestionAction::RunCommand(cmd))) = action {
            assert_eq!(cmd, "pwd");
        } else {
            panic!("Expected ApplySuggestion action");
        }
    }
} 