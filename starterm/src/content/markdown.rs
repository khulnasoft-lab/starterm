//! Provides Markdown parsing capabilities using the `pulldown-cmark` library.
//! This module is responsible for converting a Markdown string into a stream
//! of structured events (an Abstract Syntax Tree).

use pulldown_cmark::{Event, Options, Parser};

/// Parses a string containing Markdown text into a stream of events.
///
/// # Arguments
///
/// * `text` - A string slice containing the Markdown to parse.
///
/// # Returns
///
/// An iterator over the Markdown `Event`s.
pub fn parse_markdown(text: &str) -> impl Iterator<Item = Event<'_>> {
    // Enable features useful for terminals, like tables and strikethrough.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    Parser::new_ext(text, options)
}

// TODO: Integrate with `syntect` for syntax highlighting of code blocks.
// This would involve identifying `Event::Start(Tag::CodeBlock(_))` and
// passing the subsequent text content to a syntax highlighter.