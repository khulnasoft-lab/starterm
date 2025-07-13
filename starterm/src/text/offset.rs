//! Provides utilities for converting between different string offset formats,
//! such as byte-based, character-based, and grapheme-based indices. This is
//! crucial for correct cursor placement and text selection in a Unicode
//! environment.

use super::buffer::TextBuffer;

/// Represents different ways to index into a string.
pub enum OffsetKind {
    /// Index by raw bytes.
    Byte,
    /// Index by Unicode scalar values (chars).
    Char,
    /// Index by user-perceived characters (grapheme clusters).
    Grapheme,
}

/// Converts an offset from one kind to another within a given text buffer.
pub fn convert_offset(
    buffer: &TextBuffer,
    offset: usize,
    from: OffsetKind,
    to: OffsetKind,
) -> Option<usize> {
    // TODO: Implement the conversion logic. This will involve iterating
    // through the rope's chunks and using `unicode-segmentation` to
    // count graphemes when needed. This is a placeholder.
    match (from, to) {
        (OffsetKind::Byte, OffsetKind::Char) => buffer.byte_to_char(offset),
        (OffsetKind::Char, OffsetKind::Byte) => buffer.char_to_byte(offset),
        // Other conversions require more complex logic.
        _ => None,
    }
}