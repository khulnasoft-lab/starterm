use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use starterm_config_derive::ConfigDeserialize;

use crate::display::SizeInfo;
use crate::ui::block::{Block, BlockPosition, BlockSize, LayoutDirection, Alignment, PositionMode, SizeMode};

/// Layout configuration
#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Layout {
    /// Layout mode
    pub mode: LayoutMode,
    
    /// Layout direction
    pub direction: LayoutDirection,
    
    /// Alignment
    pub alignment: Alignment,
    
    /// Spacing between blocks
    pub spacing: u16,
    
    /// Padding around the layout
    pub padding: u16,
    
    /// Whether to distribute space evenly
    pub distribute: bool,
    
    /// Layout constraints
    pub constraints: Vec<LayoutConstraint>,
}

/// Layout mode
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LayoutMode {
    /// Fixed positioning
    Fixed,
    
    /// Flexbox-style layout
    Flex,
    
    /// Grid layout
    Grid(GridConfig),
    
    /// Stack layout
    Stack,
    
    /// Absolute positioning
    Absolute,
}

/// Grid configuration
#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq)]
pub struct GridConfig {
    /// Number of columns
    pub columns: u16,
    
    /// Number of rows
    pub rows: u16,
    
    /// Column gap
    pub column_gap: u16,
    
    /// Row gap
    pub row_gap: u16,
    
    /// Column template
    pub column_template: Vec<GridTrackSize>,
    
    /// Row template
    pub row_template: Vec<GridTrackSize>,
}

/// Grid track size
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GridTrackSize {
    /// Fixed size in pixels
    Fixed(u16),
    
    /// Fraction of available space
    Fraction(f32),
    
    /// Minimum size
    Min(u16),
    
    /// Maximum size
    Max(u16),
    
    /// Auto size
    Auto,
}

/// Layout constraint
#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq)]
pub struct LayoutConstraint {
    /// Block ID this constraint applies to
    pub block_id: String,
    
    /// Minimum width
    pub min_width: Option<f32>,
    
    /// Maximum width
    pub max_width: Option<f32>,
    
    /// Minimum height
    pub min_height: Option<f32>,
    
    /// Maximum height
    pub max_height: Option<f32>,
    
    /// Aspect ratio
    pub aspect_ratio: Option<f32>,
    
    /// Flex grow factor
    pub flex_grow: f32,
    
    /// Flex shrink factor
    pub flex_shrink: f32,
    
    /// Flex basis
    pub flex_basis: Option<f32>,
}

/// Layout manager
#[derive(Debug)]
pub struct LayoutManager {
    config: Layout,
    cached_layout: Option<LayoutCache>,
}

/// Cached layout information
#[derive(Debug, Clone)]
struct LayoutCache {
    size_info: SizeInfo,
    block_positions: HashMap<String, (f32, f32, f32, f32)>,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            mode: LayoutMode::Fixed,
            direction: LayoutDirection::Vertical,
            alignment: Alignment::Start,
            spacing: 4,
            padding: 8,
            distribute: false,
            constraints: Vec::new(),
        }
    }
}

impl Default for LayoutConstraint {
    fn default() -> Self {
        Self {
            block_id: String::new(),
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            aspect_ratio: None,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
        }
    }
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            columns: 1,
            rows: 1,
            column_gap: 0,
            row_gap: 0,
            column_template: Vec::new(),
            row_template: Vec::new(),
        }
    }
}

// Add SerdeReplace implementations for the enums that need them
impl starterm_config::SerdeReplace for LayoutMode {
    fn replace(&mut self, value: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
        *self = serde::Deserialize::deserialize(value)?;
        Ok(())
    }
}

impl starterm_config::SerdeReplace for GridTrackSize {
    fn replace(&mut self, value: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
        *self = serde::Deserialize::deserialize(value)?;
        Ok(())
    }
}

impl LayoutManager {
    /// Create a new layout manager
    pub fn new(config: Layout) -> Self {
        Self {
            config,
            cached_layout: None,
        }
    }
    
    /// Update the layout for all blocks
    pub fn update_layout(&mut self, size_info: &SizeInfo, blocks: &mut HashMap<String, Block>) {
        // Check if we need to recalculate layout
        let needs_update = self.cached_layout.as_ref()
            .map(|cache| cache.size_info != *size_info)
            .unwrap_or(true);
        
        if needs_update {
            self.calculate_layout(size_info, blocks);
        }
        
        // Apply cached positions to blocks
        if let Some(cache) = &self.cached_layout {
            for (block_id, (x, y, width, height)) in &cache.block_positions {
                if let Some(block) = blocks.get_mut(block_id) {
                    block.set_position(*x, *y);
                    block.set_size(*width, *height);
                }
            }
        }
    }
    
    /// Calculate layout positions for all blocks
    fn calculate_layout(&mut self, size_info: &SizeInfo, blocks: &HashMap<String, Block>) {
        let mut block_positions = HashMap::new();
        
        match self.config.mode {
            LayoutMode::Fixed => {
                self.calculate_fixed_layout(size_info, blocks, &mut block_positions);
            },
            LayoutMode::Flex => {
                self.calculate_flex_layout(size_info, blocks, &mut block_positions);
            },
            LayoutMode::Grid(ref grid_config) => {
                self.calculate_grid_layout(size_info, blocks, grid_config, &mut block_positions);
            },
            LayoutMode::Stack => {
                self.calculate_stack_layout(size_info, blocks, &mut block_positions);
            },
            LayoutMode::Absolute => {
                self.calculate_absolute_layout(size_info, blocks, &mut block_positions);
            },
        }
        
        self.cached_layout = Some(LayoutCache {
            size_info: *size_info,
            block_positions,
        });
    }
    
    /// Calculate fixed layout
    fn calculate_fixed_layout(
        &self,
        size_info: &SizeInfo,
        blocks: &HashMap<String, Block>,
        positions: &mut HashMap<String, (f32, f32, f32, f32)>,
    ) {
        // Fixed layout uses the block's configured positions
        for (block_id, block) in blocks {
            let (x, y) = self.calculate_position(&block.position, size_info);
            let (width, height) = self.calculate_size(&block.size, size_info);
            positions.insert(block_id.clone(), (x, y, width, height));
        }
    }
    
    /// Calculate flex layout
    fn calculate_flex_layout(
        &self,
        size_info: &SizeInfo,
        blocks: &HashMap<String, Block>,
        positions: &mut HashMap<String, (f32, f32, f32, f32)>,
    ) {
        let available_width = size_info.width() - 2.0 * self.config.padding as f32;
        let available_height = size_info.height() - 2.0 * self.config.padding as f32;
        
        let mut sorted_blocks: Vec<_> = blocks.iter().collect();
        sorted_blocks.sort_by_key(|(_, block)| block.z_order);
        
        let mut current_x = self.config.padding as f32;
        let mut current_y = self.config.padding as f32;
        
        for (block_id, block) in sorted_blocks {
            let (width, height) = self.calculate_size(&block.size, size_info);
            
            // Apply flex constraints
            let constraint = self.config.constraints.iter()
                .find(|c| c.block_id == *block_id);
            
            let final_width = if let Some(constraint) = constraint {
                width.max(constraint.min_width.unwrap_or(0.0))
                     .min(constraint.max_width.unwrap_or(f32::MAX))
            } else {
                width
            };
            
            let final_height = if let Some(constraint) = constraint {
                height.max(constraint.min_height.unwrap_or(0.0))
                      .min(constraint.max_height.unwrap_or(f32::MAX))
            } else {
                height
            };
            
            match self.config.direction {
                LayoutDirection::Horizontal => {
                    if current_x + final_width > available_width {
                        current_x = self.config.padding as f32;
                        current_y += final_height + self.config.spacing as f32;
                    }
                    
                    positions.insert(block_id.clone(), (current_x, current_y, final_width, final_height));
                    current_x += final_width + self.config.spacing as f32;
                },
                LayoutDirection::Vertical => {
                    if current_y + final_height > available_height {
                        current_y = self.config.padding as f32;
                        current_x += final_width + self.config.spacing as f32;
                    }
                    
                    positions.insert(block_id.clone(), (current_x, current_y, final_width, final_height));
                    current_y += final_height + self.config.spacing as f32;
                },
            }
        }
    }
    
    /// Calculate grid layout
    fn calculate_grid_layout(
        &self,
        size_info: &SizeInfo,
        blocks: &HashMap<String, Block>,
        grid_config: &GridConfig,
        positions: &mut HashMap<String, (f32, f32, f32, f32)>,
    ) {
        let available_width = size_info.width() - 2.0 * self.config.padding as f32;
        let available_height = size_info.height() - 2.0 * self.config.padding as f32;
        
        // Calculate column widths
        let column_widths = self.calculate_track_sizes(
            &grid_config.column_template,
            available_width,
            grid_config.columns,
            grid_config.column_gap,
        );
        
        // Calculate row heights
        let row_heights = self.calculate_track_sizes(
            &grid_config.row_template,
            available_height,
            grid_config.rows,
            grid_config.row_gap,
        );
        
        let mut sorted_blocks: Vec<_> = blocks.iter().collect();
        sorted_blocks.sort_by_key(|(_, block)| block.z_order);
        
        let mut current_row = 0;
        let mut current_col = 0;
        
        for (block_id, _) in sorted_blocks {
            if current_col >= grid_config.columns {
                current_col = 0;
                current_row += 1;
            }
            
            if current_row >= grid_config.rows {
                break;
            }
            
            let x = self.config.padding as f32 + 
                    (0..current_col).map(|i| column_widths[i as usize] + grid_config.column_gap as f32).sum::<f32>();
            let y = self.config.padding as f32 + 
                    (0..current_row).map(|i| row_heights[i as usize] + grid_config.row_gap as f32).sum::<f32>();
            
            let width = column_widths[current_col as usize];
            let height = row_heights[current_row as usize];
            
            positions.insert(block_id.clone(), (x, y, width, height));
            
            current_col += 1;
        }
    }
    
    /// Calculate stack layout
    fn calculate_stack_layout(
        &self,
        size_info: &SizeInfo,
        blocks: &HashMap<String, Block>,
        positions: &mut HashMap<String, (f32, f32, f32, f32)>,
    ) {
        let available_width = size_info.width() - 2.0 * self.config.padding as f32;
        let available_height = size_info.height() - 2.0 * self.config.padding as f32;
        
        // In stack layout, all blocks occupy the same space
        for (block_id, _) in blocks {
            positions.insert(
                block_id.clone(),
                (
                    self.config.padding as f32,
                    self.config.padding as f32,
                    available_width,
                    available_height,
                ),
            );
        }
    }
    
    /// Calculate absolute layout
    fn calculate_absolute_layout(
        &self,
        size_info: &SizeInfo,
        blocks: &HashMap<String, Block>,
        positions: &mut HashMap<String, (f32, f32, f32, f32)>,
    ) {
        // Absolute layout uses exact positions without any automatic layouting
        for (block_id, block) in blocks {
            let (x, y) = self.calculate_position(&block.position, size_info);
            let (width, height) = self.calculate_size(&block.size, size_info);
            positions.insert(block_id.clone(), (x, y, width, height));
        }
    }
    
    /// Calculate track sizes for grid layout
    fn calculate_track_sizes(
        &self,
        template: &[GridTrackSize],
        available_space: f32,
        track_count: u16,
        gap: u16,
    ) -> Vec<f32> {
        let mut sizes = Vec::with_capacity(track_count as usize);
        let total_gap = (track_count.saturating_sub(1)) as f32 * gap as f32;
        let remaining_space = available_space - total_gap;
        
        if template.is_empty() {
            // Equal distribution
            let size = remaining_space / track_count as f32;
            sizes.resize(track_count as usize, size);
        } else {
            // Use template
            let mut fraction_sum = 0.0;
            let mut fixed_sum = 0.0;
            
            for track_size in template {
                match track_size {
                    GridTrackSize::Fixed(pixels) => fixed_sum += *pixels as f32,
                    GridTrackSize::Fraction(fraction) => fraction_sum += fraction,
                    GridTrackSize::Auto => fixed_sum += 100.0, // Default auto size
                    GridTrackSize::Min(pixels) => fixed_sum += *pixels as f32,
                    GridTrackSize::Max(pixels) => fixed_sum += *pixels as f32,
                }
            }
            
            let fraction_space = (remaining_space - fixed_sum).max(0.0);
            
            for track_size in template {
                let size = match track_size {
                    GridTrackSize::Fixed(pixels) => *pixels as f32,
                    GridTrackSize::Fraction(fraction) => {
                        if fraction_sum > 0.0 {
                            fraction_space * fraction / fraction_sum
                        } else {
                            0.0
                        }
                    },
                    GridTrackSize::Auto => 100.0,
                    GridTrackSize::Min(pixels) => *pixels as f32,
                    GridTrackSize::Max(pixels) => *pixels as f32,
                };
                sizes.push(size);
            }
            
            // Fill remaining tracks with equal distribution
            while sizes.len() < track_count as usize {
                sizes.push(remaining_space / track_count as f32);
            }
        }
        
        sizes
    }
    
    /// Calculate position from block position configuration
    fn calculate_position(&self, position: &BlockPosition, size_info: &SizeInfo) -> (f32, f32) {
        match position.mode {
            PositionMode::Absolute => (position.x, position.y),
            PositionMode::Relative => (
                position.x * size_info.width() / 100.0,
                position.y * size_info.height() / 100.0,
            ),
            PositionMode::Fixed => (position.x, position.y),
        }
    }
    
    /// Calculate size from block size configuration
    fn calculate_size(&self, size: &BlockSize, size_info: &SizeInfo) -> (f32, f32) {
        match size.mode {
            SizeMode::Fixed => (size.width, size.height),
            SizeMode::Relative => (
                size.width * size_info.width() / 100.0,
                size.height * size_info.height() / 100.0,
            ),
            SizeMode::Auto => (100.0, 50.0), // Default auto size
            SizeMode::Fill => (size_info.width(), size_info.height()),
        }
    }
    
    /// Update layout configuration
    pub fn update_config(&mut self, config: Layout) {
        self.config = config;
        self.cached_layout = None; // Invalidate cache
    }
}
