//! Renders a stream of Markdown events into styled terminal output.
//! (Note: This file is placed in `ui` as per the plan, create the directory if it doesn't exist)

use pulldown_cmark::Event;

/// Represents a segment of styled text for terminal rendering.
/// The `style_id` would correspond to a style defined in the terminal's theme.
pub struct StyledText {
    pub content: String,
    pub style_id: String, // e.g., "markdown.h1", "markdown.code"
}

/// A renderer that consumes Markdown events and produces styled text.
pub struct MarkdownRenderer;

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self
    }

    /// Renders a stream of events into a vector of `StyledText`.
    ///
    /// This output can then be translated into ANSI escape codes or drawn
    /// directly to the terminal's cell grid.
    pub fn render<'a>(&self, events: impl Iterator<Item = Event<'a>>) -> Vec<StyledText> {
        let mut styled_text_list = Vec::new();
        let mut current_style = "markdown.body".to_string();

        for event in events {
            match event {
                Event::Start(tag) => {
                    // TODO: Set current_style based on the tag (e.g., Heading, Code, etc.)
                }
                Event::End(tag) => {
                    // TODO: Revert to the previous style.
                    // This will require a style stack.
                }
                Event::Text(text) => {
                    styled_text_list.push(StyledText {
                        content: text.into_string(),
                        style_id: current_style.clone(),
                    });
                }
                Event::Code(text) => {
                    styled_text_list.push(StyledText {
                        content: text.into_string(),
                        style_id: "markdown.inline_code".to_string(),
                    });
                }
                // TODO: Handle other event types like SoftBreak, HardBreak, Rule, etc.
                _ => (),
            }
        }
        styled_text_list
    }
}