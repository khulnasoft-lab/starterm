// starterm/src/lib.rs (or main.rs)

// --- Phase 1 Modules ---
pub mod input;
pub mod search;
pub mod text;

// --- Phase 2 Modules ---
pub mod completion;
pub mod content;
pub mod fs;
pub mod ui;

// --- Phase 3 Modules ---
pub mod analysis;
pub mod nlp;
pub mod validation;

// --- Phase 4 Modules ---
pub mod assets;
pub mod data;
pub mod lpc;

// --- Phase 5 Modules ---
pub mod ai;

// --- Core Modules ---
pub mod cli;
pub mod clipboard;
pub mod config;
pub mod daemon;
pub mod display;
pub mod event;
pub mod ipc;
pub mod logging;
pub mod macos;
pub mod message_bar;
pub mod renderer;
pub mod scheduler;
pub mod string;
pub mod window_context;
pub mod workflows;