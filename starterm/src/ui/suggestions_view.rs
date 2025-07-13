//! A view component for rendering the AI suggestions overlay.

use super::state::SuggestionsState;
// Note: We'd use a real styled text struct from the renderer here.
// Using a simple tuple for demonstration.
pub type StyledLine = (String, String); // (content, style_id)

pub struct SuggestionsView;

impl SuggestionsView {
    /// Renders the suggestions state into a list of styled lines.
    /// These lines can then be drawn by the main terminal renderer.
    pub fn render(state: &SuggestionsState) -> Vec<StyledLine> {
        let mut lines = Vec::new();
        lines.push(("AI Suggestions:".to_string(), "style.suggestion.header".to_string()));

        for (i, suggestion) in state.suggestions.iter().enumerate() {
            let (prefix, style) = if i == state.selected_index {
                ("> ".to_string(), "style.suggestion.selected".to_string())
            } else {
                ("  ".to_string(), "style.suggestion.default".to_string())
            };
            lines.push((format!("{}{}", prefix, suggestion.text), style));
        }
        lines
    }
} 