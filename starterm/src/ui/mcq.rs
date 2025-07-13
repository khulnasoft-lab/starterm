//! A UI component for rendering and handling Multiple Choice Questions.

// Note: This is a placeholder for the MCQ functionality
// The actual implementation will need to be integrated with the existing workflows system

/// Represents a single choice in a multiple-choice question.
#[derive(Debug, Clone)]
pub struct McqChoice {
    pub display_text: String,
    pub value: String,
}

/// Represents a request for user input via a multiple-choice question.
#[derive(Debug, Clone)]
pub struct McqStep {
    pub id: String,
    pub prompt: String,
    pub choices: Vec<McqChoice>,
    pub validation_rules: Vec<String>, // Simplified for now
}

/// The state of an interactive MCQ session.
pub struct McqComponent {
    config: McqStep,
    selected_index: usize,
    is_active: bool,
}

impl McqComponent {
    pub fn new(config: McqStep) -> Self {
        Self {
            config,
            selected_index: 0,
            is_active: true,
        }
    }

    /// Renders the MCQ to a list of styled strings for the terminal UI.
    pub fn render(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(self.config.prompt.clone());
        for (i, choice) in self.config.choices.iter().enumerate() {
            let prefix = if i == self.selected_index { "> " } else { "  " };
            lines.push(format!("{}{}", prefix, choice.display_text));
        }
        lines
    }

    /// Handles user input to navigate the choices.
    pub fn handle_input(&mut self, key: &str) {
        match key {
            "up" if self.selected_index > 0 => self.selected_index -= 1,
            "down" if self.selected_index < self.config.choices.len() - 1 => {
                self.selected_index += 1;
            }
            _ => {}
        }
    }

    /// Finalizes the selection and returns the chosen value.
    pub fn get_selected_value(&self) -> Option<&str> {
        self.config
            .choices
            .get(self.selected_index)
            .map(|c| c.value.as_str())
    }
} 