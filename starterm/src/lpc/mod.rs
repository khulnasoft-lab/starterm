//! LPC (Language Processing Core) for cross-shell command understanding and translation.
//!
//! This module provides the engine for parsing a command from one shell (e.g., bash),
//! representing it in an abstract model, and then translating it to another
//! shell's syntax (e.g., PowerShell).

pub mod adapters;
pub mod core;
pub mod translator;
pub mod validator; 