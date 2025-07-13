//! An efficient text buffer implementation using a rope data structure.
//! This is a core component for handling large amounts of text, such as
//! terminal scrollback history, with high performance.

use ropey::Rope;
use std::io;

/// A memory-efficient text buffer optimized for large strings and frequent edits.
pub struct TextBuffer {
    rope: Rope,
}

impl TextBuffer {
    /// Creates a new, empty text buffer.
    pub fn new() -> Self {
        Self { rope: Rope::new() }
    }

    /// Creates a text buffer from a string slice.
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
        }
    }

    /// Inserts text at a specific byte offset.
    pub fn insert(&mut self, byte_idx: usize, text: &str) {
        // TODO: Add robust error handling for out-of-bounds indices.
        let char_idx = self.rope.byte_to_char(byte_idx);
        self.rope.insert(char_idx, text);
    }

    /// Deletes a range of text specified by byte offsets.
    pub fn delete(&mut self, byte_range: std::ops::Range<usize>) {
        // TODO: Add robust error handling.
        let start_char = self.rope.byte_to_char(byte_range.start);
        let end_char = self.rope.byte_to_char(byte_range.end);
        self.rope.remove(start_char..end_char);
    }

    /// Returns the total number of bytes in the buffer.
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Returns an iterator over the lines of the buffer.
    pub fn lines(&self) -> impl Iterator<Item = ropey::RopeSlice<'_>> {
        self.rope.lines()
    }

    /// Writes the buffer's contents to a writer.
    /// This can be used for persistence or exporting terminal content.
    pub fn write_to<W: io::Write>(&self, writer: W) -> io::Result<()> {
        self.rope.write_to(writer)
    }

    /// Converts a byte index to a character index.
    pub fn byte_to_char(&self, byte_idx: usize) -> Option<usize> {
        Some(self.rope.byte_to_char(byte_idx))
    }

    /// Converts a character index to a byte index.
    pub fn char_to_byte(&self, char_idx: usize) -> Option<usize> {
        Some(self.rope.char_to_byte(char_idx))
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}