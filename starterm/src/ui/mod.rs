use std::collections::HashMap;
use std::fmt;

use serde::Serialize;
use starterm_config_derive::ConfigDeserialize;

use crate::display::color::Rgb;
use crate::display::SizeInfo;

pub mod block;
pub mod layout;
pub mod renderer;
pub mod workflows_sidebar;
pub mod syntax_highlight;
pub mod mcq;
pub mod state;
pub mod suggestions_view;

pub use block::{Block, BlockConfig};
pub use layout::{Layout, LayoutManager};
pub use workflows_sidebar::WorkflowsSidebar;

/// Configuration for the UI system
#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UiSystemConfig {
    /// Whether the UI system is enabled
    pub enabled: bool,
    
    /// Global UI theme settings
    pub theme: UiTheme,
    
    /// Block configurations
    pub blocks: HashMap<String, BlockConfig>,
    
    /// Layout configuration
    pub layout: Layout,
}

impl Default for UiSystemConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            theme: UiTheme::default(),
            blocks: HashMap::new(),
            layout: Layout::default(),
        }
    }
}

/// UI theme configuration
#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UiTheme {
    /// Default background color for UI blocks
    pub background: Rgb,
    
    /// Default foreground color for UI blocks
    pub foreground: Rgb,
    
    /// Default border color
    pub border: Rgb,
    
    /// Accent color for highlights
    pub accent: Rgb,
    
    /// Error color
    pub error: Rgb,
    
    /// Warning color
    pub warning: Rgb,
    
    /// Success color
    pub success: Rgb,
    
    /// Default padding inside blocks
    pub padding: u16,
    
    /// Default border width
    pub border_width: u16,
    
    /// Default corner radius
    pub corner_radius: u16,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            background: Rgb::new(40, 44, 52),      // Dark background
            foreground: Rgb::new(171, 178, 191),   // Light text
            border: Rgb::new(75, 81, 96),          // Subtle border
            accent: Rgb::new(97, 175, 239),        // Blue accent
            error: Rgb::new(224, 108, 117),        // Red error
            warning: Rgb::new(229, 192, 123),      // Yellow warning
            success: Rgb::new(152, 195, 121),      // Green success
            padding: 8,
            border_width: 1,
            corner_radius: 4,
        }
    }
}

/// UI system state and management
#[derive(Debug)]
pub struct UiSystem {
    config: UiSystemConfig,
    layout_manager: LayoutManager,
    blocks: HashMap<String, Block>,
    workflows_sidebar: Option<WorkflowsSidebar>,
    needs_redraw: bool,
}

impl UiSystem {
    /// Create a new UI system with the given configuration
    pub fn new(config: UiSystemConfig) -> Self {
        let layout_manager = LayoutManager::new(config.layout.clone());
        
        Self {
            config,
            layout_manager,
            blocks: HashMap::new(),
            workflows_sidebar: None,
            needs_redraw: true,
        }
    }
    
    /// Update the UI system configuration
    pub fn update_config(&mut self, config: UiSystemConfig) {
        self.config = config;
        self.needs_redraw = true;
    }
    
    /// Add a block to the UI system
    pub fn add_block(&mut self, id: String, block: Block) {
        self.blocks.insert(id, block);
        self.needs_redraw = true;
    }
    
    /// Remove a block from the UI system
    pub fn remove_block(&mut self, id: &str) -> Option<Block> {
        let removed = self.blocks.remove(id);
        if removed.is_some() {
            self.needs_redraw = true;
        }
        removed
    }
    
    /// Get a mutable reference to a block
    pub fn get_block_mut(&mut self, id: &str) -> Option<&mut Block> {
        self.blocks.get_mut(id)
    }
    
    /// Get a reference to a block
    pub fn get_block(&self, id: &str) -> Option<&Block> {
        self.blocks.get(id)
    }
    
    /// Update the layout
    pub fn update_layout(&mut self, size_info: &SizeInfo) {
        self.layout_manager.update_layout(size_info, &mut self.blocks);
        self.needs_redraw = true;
    }
    
    /// Check if the UI needs to be redrawn
    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw || self.blocks.values().any(|block| block.needs_redraw())
    }
    
    /// Mark the UI as redrawn
    pub fn mark_redrawn(&mut self) {
        self.needs_redraw = false;
        for block in self.blocks.values_mut() {
            block.mark_redrawn();
        }
    }
    
    /// Get all blocks for rendering
    pub fn blocks(&self) -> &HashMap<String, Block> {
        &self.blocks
    }
    
    /// Get the UI theme
    pub fn theme(&self) -> &UiTheme {
        &self.config.theme
    }
    
    /// Check if the UI system is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
    
    /// Enable workflows sidebar
    pub fn enable_workflows_sidebar(&mut self) {
        if self.workflows_sidebar.is_none() {
            let mut sidebar = WorkflowsSidebar::new();
            sidebar.apply_theme(&self.config.theme);
            self.workflows_sidebar = Some(sidebar);
            self.needs_redraw = true;
        }
    }
    
    /// Disable workflows sidebar
    pub fn disable_workflows_sidebar(&mut self) {
        if self.workflows_sidebar.is_some() {
            self.workflows_sidebar = None;
            self.needs_redraw = true;
        }
    }
    
    /// Toggle workflows sidebar
    pub fn toggle_workflows_sidebar(&mut self) {
        if self.workflows_sidebar.is_some() {
            self.disable_workflows_sidebar();
        } else {
            self.enable_workflows_sidebar();
        }
    }
    
    /// Get workflows sidebar reference
    pub fn workflows_sidebar(&self) -> Option<&WorkflowsSidebar> {
        self.workflows_sidebar.as_ref()
    }
    
    /// Get mutable workflows sidebar reference
    pub fn workflows_sidebar_mut(&mut self) -> Option<&mut WorkflowsSidebar> {
        self.workflows_sidebar.as_mut()
    }
    
    /// Check if workflows sidebar is enabled
    pub fn is_workflows_sidebar_enabled(&self) -> bool {
        self.workflows_sidebar.is_some()
    }
    
    /// Handle click event for workflows sidebar
    pub fn handle_sidebar_click(&mut self, x: f32, y: f32) -> Option<String> {
        if let Some(sidebar) = &mut self.workflows_sidebar {
            sidebar.handle_click(x, y)
        } else {
            None
        }
    }
    
    /// Handle key press for workflows sidebar
    pub fn handle_sidebar_key_press(&mut self, key: &str) -> Option<String> {
        if let Some(sidebar) = &mut self.workflows_sidebar {
            sidebar.handle_key_press(key)
        } else {
            None
        }
    }
}

/// Error types for the UI system
#[derive(Debug)]
pub enum UiError {
    /// Block not found
    BlockNotFound(String),
    
    /// Invalid block configuration
    InvalidBlockConfig(String),
    
    /// Layout error
    LayoutError(String),
    
    /// Rendering error
    RenderError(String),
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiError::BlockNotFound(id) => write!(f, "Block not found: {}", id),
            UiError::InvalidBlockConfig(msg) => write!(f, "Invalid block configuration: {}", msg),
            UiError::LayoutError(msg) => write!(f, "Layout error: {}", msg),
            UiError::RenderError(msg) => write!(f, "Render error: {}", msg),
        }
    }
}

impl std::error::Error for UiError {}

/// Result type for UI operations
pub type UiResult<T> = Result<T, UiError>;
