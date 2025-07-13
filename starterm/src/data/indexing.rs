//! Defines the metrics that can be indexed by the Sum Tree.

/// A trait for any metric that can be measured on a string.
pub trait TextMetric: Send + Sync {
    /// The type of the metric's value (e.g., u64).
    type Value;

    /// Calculates the metric for a given string slice.
    fn measure(&self, text: &str) -> Self::Value;
}

/// A metric that counts the number of newline characters.
pub struct LineCountMetric;
impl TextMetric for LineCountMetric {
    type Value = u64;
    fn measure(&self, text: &str) -> Self::Value {
        bytecount::count(text.as_bytes(), b'\n') as u64
    }
} 