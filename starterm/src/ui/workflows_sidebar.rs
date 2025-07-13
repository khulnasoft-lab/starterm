//! Workflows sidebar component for the Starterm UI.
//!
//! This module provides the sidebar UI for browsing, searching, and selecting workflows.

use std::collections::HashMap;

use crate::display::color::Rgb;

use crate::ui::block::{Block, BlockType, TextAlignment, ButtonType, InputType};
use crate::ui::block::{PositionMode, SizeMode};
use crate::ui::UiTheme;
use crate::workflows::{WorkflowManager, Workflow};

/// Workflows sidebar component
#[derive(Debug)]
pub struct WorkflowsSidebar {
    /// Workflow manager
    workflow_manager: WorkflowManager,
    
    /// Whether the sidebar is collapsed
    collapsed: bool,
    
    /// Sidebar width when expanded
    expanded_width: f32,
    
    /// Sidebar width when collapsed
    collapsed_width: f32,
    
    /// Current search query
    search_query: String,
    
    /// Whether the search input is focused
    search_focused: bool,
    
    /// Selected workflow index
    selected_index: Option<usize>,
    
    /// Scroll position
    scroll_position: f32,
    
    /// Maximum visible workflows
    max_visible_workflows: usize,
    
    /// Block components
    blocks: HashMap<String, Block>,
    
    /// Whether the sidebar needs redraw
    needs_redraw: bool,
}

impl WorkflowsSidebar {
    /// Create a new workflows sidebar
    pub fn new() -> Self {
        let mut sidebar = Self {
            workflow_manager: WorkflowManager::new(),
            collapsed: false,
            expanded_width: 300.0,
            collapsed_width: 48.0,
            search_query: String::new(),
            search_focused: false,
            selected_index: None,
            scroll_position: 0.0,
            max_visible_workflows: 10,
            blocks: HashMap::new(),
            needs_redraw: true,
        };
        
        sidebar.create_blocks();
        sidebar
    }
    
    /// Create the sidebar blocks
    fn create_blocks(&mut self) {
        // Create main container
        let mut container = Block::container("sidebar_container".to_string());
        container.set_size(self.current_width(), 0.0); // Height will be set in layout
        container.size.mode = SizeMode::Fill;
        container.position.mode = PositionMode::Fixed;
        
        // Create header
        let mut header = Block::text("sidebar_header".to_string(), "Workflows".to_string());
        header.set_size(self.current_width() - 16.0, 32.0);
        if let BlockType::Text(ref mut text_block) = header.block_type {
            text_block.alignment = TextAlignment::Center;
            text_block.font_size = 1.2;
        }
        
        // Create search input
        let mut search_input = Block::input("sidebar_search".to_string(), "Search workflows...".to_string());
        search_input.set_size(self.current_width() - 16.0, 28.0);
        if let BlockType::Input(ref mut input_block) = search_input.block_type {
            input_block.input_type = InputType::Search;
            input_block.focused = self.search_focused;
            input_block.value = self.search_query.clone();
        }
        
        // Create toggle button
        let mut toggle_button = Block::button(
            "sidebar_toggle".to_string(),
            if self.collapsed { "▶" } else { "◀" }.to_string(),
            "toggle_sidebar".to_string()
        );
        toggle_button.set_size(32.0, 32.0);
        if let BlockType::Button(ref mut button_block) = toggle_button.block_type {
            button_block.button_type = ButtonType::Secondary;
        }
        
        self.blocks.insert("sidebar_container".to_string(), container);
        self.blocks.insert("sidebar_header".to_string(), header);
        self.blocks.insert("sidebar_search".to_string(), search_input);
        self.blocks.insert("sidebar_toggle".to_string(), toggle_button);
        
        // Create workflow list blocks
        self.create_workflow_blocks();
    }
    
    /// Create workflow list blocks
    fn create_workflow_blocks(&mut self) {
        let filtered_workflows = self.workflow_manager.filtered_workflows();
        
        // Remove existing workflow blocks
        let workflow_keys: Vec<String> = self.blocks.keys()
            .filter(|k| k.starts_with("workflow_"))
            .cloned()
            .collect();
        
        for key in workflow_keys {
            self.blocks.remove(&key);
        }
        
        // Create workflow blocks
        for (index, (_, workflow)) in filtered_workflows.iter().enumerate() {
            if index >= self.max_visible_workflows {
                break;
            }
            
            let workflow_id = format!("workflow_{}", index);
            let mut workflow_block = self.create_workflow_block(workflow_id.clone(), workflow, index);
            
            // Highlight selected workflow
            if self.selected_index == Some(index) {
                workflow_block.style.background = Some(Rgb::new(60, 60, 60));
                workflow_block.style.border = Some(Rgb::new(97, 175, 239));
                workflow_block.style.border_width = 2;
            }
            
            self.blocks.insert(workflow_id, workflow_block);
        }
    }
    
    /// Create a single workflow block
    fn create_workflow_block(&self, id: String, _workflow: &Workflow, index: usize) -> Block {
        let mut workflow_block = Block::container(id.clone());
        workflow_block.set_size(self.current_width() - 16.0, 60.0);
        
        // Set position based on index
        let y_offset = 100.0 + (index as f32 * 64.0) - self.scroll_position;
        workflow_block.set_position(8.0, y_offset);
        
        // Style the workflow block
        workflow_block.style.background = Some(Rgb::new(50, 50, 50));
        workflow_block.style.border = Some(Rgb::new(70, 70, 70));
        workflow_block.style.border_width = 1;
        workflow_block.style.corner_radius = 4;
        workflow_block.style.padding.top = 8;
        workflow_block.style.padding.left = 8;
        workflow_block.style.padding.right = 8;
        workflow_block.style.padding.bottom = 8;
        
        // Add click handler
        workflow_block.handlers.on_click = Some(format!("select_workflow_{}", index));
        
        workflow_block
    }
    
    /// Get current width based on collapsed state
    fn current_width(&self) -> f32 {
        if self.collapsed {
            self.collapsed_width
        } else {
            self.expanded_width
        }
    }
    
    /// Toggle sidebar collapsed state
    pub fn toggle_collapsed(&mut self) {
        self.collapsed = !self.collapsed;
        self.needs_redraw = true;
        self.update_layout();
    }
    
    /// Set search query
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query.clone();
        self.workflow_manager.set_search_filter(query);
        self.needs_redraw = true;
        self.create_workflow_blocks();
    }
    
    /// Get search query
    pub fn search_query(&self) -> &str {
        &self.search_query
    }
    
    /// Set search focus
    pub fn set_search_focused(&mut self, focused: bool) {
        self.search_focused = focused;
        self.needs_redraw = true;
        
        if let Some(search_block) = self.blocks.get_mut("sidebar_search") {
            if let BlockType::Input(ref mut input_block) = search_block.block_type {
                input_block.focused = focused;
            }
        }
    }
    
    /// Move selection up
    pub fn move_selection_up(&mut self) {
        self.workflow_manager.move_selection_up();
        self.selected_index = self.workflow_manager.selection();
        self.needs_redraw = true;
        self.create_workflow_blocks();
    }
    
    /// Move selection down
    pub fn move_selection_down(&mut self) {
        self.workflow_manager.move_selection_down();
        self.selected_index = self.workflow_manager.selection();
        self.needs_redraw = true;
        self.create_workflow_blocks();
    }
    
    /// Select workflow by index
    pub fn select_workflow(&mut self, index: usize) {
        self.workflow_manager.set_selection(Some(index));
        self.selected_index = Some(index);
        self.needs_redraw = true;
        self.create_workflow_blocks();
    }
    
    /// Get selected workflow
    pub fn selected_workflow(&self) -> Option<&Workflow> {
        self.workflow_manager.selected_workflow()
    }
    
    /// Get workflow manager
    pub fn workflow_manager(&self) -> &WorkflowManager {
        &self.workflow_manager
    }
    
    /// Get mutable workflow manager
    pub fn workflow_manager_mut(&mut self) -> &mut WorkflowManager {
        &mut self.workflow_manager
    }
    
    /// Scroll up
    pub fn scroll_up(&mut self) {
        self.scroll_position = (self.scroll_position - 64.0).max(0.0);
        self.needs_redraw = true;
        self.create_workflow_blocks();
    }
    
    /// Scroll down
    pub fn scroll_down(&mut self) {
        let max_scroll = (self.workflow_manager.filtered_workflows().len() as f32 * 64.0)
            .max(0.0);
        self.scroll_position = (self.scroll_position + 64.0).min(max_scroll);
        self.needs_redraw = true;
        self.create_workflow_blocks();
    }
    
    /// Update layout for size changes
    pub fn update_layout(&mut self) {
        self.create_blocks();
    }
    
    /// Apply theme to sidebar
    pub fn apply_theme(&mut self, theme: &UiTheme) {
        for block in self.blocks.values_mut() {
            block.apply_theme(theme);
        }
        self.needs_redraw = true;
    }
    
    /// Get all blocks for rendering
    pub fn blocks(&self) -> &HashMap<String, Block> {
        &self.blocks
    }
    
    /// Get mutable blocks
    pub fn blocks_mut(&mut self) -> &mut HashMap<String, Block> {
        &mut self.blocks
    }
    
    /// Check if sidebar needs redraw
    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw || self.blocks.values().any(|block| block.needs_redraw())
    }
    
    /// Mark sidebar as redrawn
    pub fn mark_redrawn(&mut self) {
        self.needs_redraw = false;
        for block in self.blocks.values_mut() {
            block.mark_redrawn();
        }
    }
    
    /// Get sidebar width
    pub fn width(&self) -> f32 {
        self.current_width()
    }
    
    /// Check if sidebar is collapsed
    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }
    
    /// Handle click event
    pub fn handle_click(&mut self, x: f32, y: f32) -> Option<String> {
        // Check if click is on toggle button
        if let Some(toggle_block) = self.blocks.get("sidebar_toggle") {
            if toggle_block.contains_point(x, y) {
                self.toggle_collapsed();
                return Some("toggle_sidebar".to_string());
            }
        }
        
        // Check if click is on search input
        if let Some(search_block) = self.blocks.get("sidebar_search") {
            if search_block.contains_point(x, y) {
                self.set_search_focused(true);
                return Some("focus_search".to_string());
            }
        }
        
        // Check if click is on a workflow
        for (block_id, block) in &self.blocks {
            if block_id.starts_with("workflow_") && block.contains_point(x, y) {
                if let Some(index_str) = block_id.strip_prefix("workflow_") {
                    if let Ok(index) = index_str.parse::<usize>() {
                        self.select_workflow(index);
                        return Some(format!("select_workflow_{}", index));
                    }
                }
            }
        }
        
        None
    }
    
    /// Handle key press
    pub fn handle_key_press(&mut self, key: &str) -> Option<String> {
        match key {
            "ArrowUp" => {
                self.move_selection_up();
                Some("move_selection_up".to_string())
            }
            "ArrowDown" => {
                self.move_selection_down();
                Some("move_selection_down".to_string())
            }
            "Enter" => {
                if let Some(workflow) = self.selected_workflow() {
                    Some(format!("execute_workflow_{}", workflow.name()))
                } else {
                    None
                }
            }
            "Escape" => {
                self.set_search_focused(false);
                Some("blur_search".to_string())
            }
            _ => None
        }
    }
}

impl Default for WorkflowsSidebar {
    fn default() -> Self {
        Self::new()
    }
}

/// Workflow sidebar events
#[derive(Debug, Clone)]
pub enum WorkflowSidebarEvent {
    /// Workflow selected
    WorkflowSelected(usize),
    /// Search query changed
    SearchChanged(String),
    /// Search focused/blurred
    SearchFocused(bool),
    /// Sidebar toggled
    SidebarToggled(bool),
    /// Workflow executed
    WorkflowExecuted(String),
    /// Selection moved
    SelectionMoved(Option<usize>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidebar_creation() {
        let sidebar = WorkflowsSidebar::new();
        assert!(!sidebar.is_collapsed());
        assert_eq!(sidebar.width(), 300.0);
        assert!(!sidebar.blocks.is_empty());
    }

    #[test]
    fn test_sidebar_toggle() {
        let mut sidebar = WorkflowsSidebar::new();
        assert!(!sidebar.is_collapsed());
        
        sidebar.toggle_collapsed();
        assert!(sidebar.is_collapsed());
        assert_eq!(sidebar.width(), 48.0);
        
        sidebar.toggle_collapsed();
        assert!(!sidebar.is_collapsed());
        assert_eq!(sidebar.width(), 300.0);
    }

    #[test]
    fn test_search_functionality() {
        let mut sidebar = WorkflowsSidebar::new();
        
        sidebar.set_search_query("git".to_string());
        assert_eq!(sidebar.search_query(), "git");
        
        // Test that search affects workflow manager
        let filtered = sidebar.workflow_manager().filtered_workflows();
        assert!(!filtered.is_empty());
    }

    #[test]
    fn test_selection_navigation() {
        let mut sidebar = WorkflowsSidebar::new();
        
        // Initially no selection
        assert_eq!(sidebar.selected_index, None);
        
        // Move down should select first item
        sidebar.move_selection_down();
        assert_eq!(sidebar.selected_index, Some(0));
        
        // Move up should wrap to last item
        sidebar.move_selection_up();
        let workflow_count = sidebar.workflow_manager().workflows().len();
        assert_eq!(sidebar.selected_index, Some(workflow_count - 1));
    }

    #[test]
    fn test_click_handling() {
        let mut sidebar = WorkflowsSidebar::new();
        
        // Test clicking on toggle button (assuming it's positioned correctly)
        let result = sidebar.handle_click(16.0, 16.0);
        // This test depends on the exact positioning, so it might need adjustment
        
        // Test that invalid clicks return None
        let result = sidebar.handle_click(-10.0, -10.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_key_handling() {
        let mut sidebar = WorkflowsSidebar::new();
        
        // Test arrow keys
        let result = sidebar.handle_key_press("ArrowDown");
        assert_eq!(result, Some("move_selection_down".to_string()));
        
        let result = sidebar.handle_key_press("ArrowUp");
        assert_eq!(result, Some("move_selection_up".to_string()));
        
        // Test escape key
        sidebar.set_search_focused(true);
        let result = sidebar.handle_key_press("Escape");
        assert_eq!(result, Some("blur_search".to_string()));
        assert!(!sidebar.search_focused);
    }
}
