//! The autocompletion engine for Starterm.
//!
//! This module provides a flexible, provider-based architecture for generating
//! command, flag, and file path completions.

pub mod cache;
pub mod engine;
pub mod providers;