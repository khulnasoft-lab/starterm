use std::fmt;

use serde::{Deserialize, Serialize};
use starterm_config_derive::ConfigDeserialize;

use crate::display::color::Rgb;
use crate::ui::UiTheme;

/// A UI block that can be rendered in the terminal window
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// Block type and content
    pub block_type: BlockType,
    
    /// Block position
    pub position: BlockPosition,
    
    /// Block size
    pub size: BlockSize,
    
    /// Block styling
    pub style: BlockStyle,
    
    /// Whether the block is visible
    pub visible: bool,
    
    /// Whether the block needs to be redrawn
    needs_redraw: bool,
    
    /// Block ID for identification
    pub id: String,
    
    /// Z-order for layering
    pub z_order: i32,
    
    /// Event handlers
    pub handlers: BlockHandlers,
}

/// Block type and content
#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    /// Text block
    Text(TextBlock),
    
    /// Button block
    Button(ButtonBlock),
    
    /// Image block
    Image(ImageBlock),
    
    /// Container block that can hold other blocks
    Container(ContainerBlock),
    
    /// Progress bar block
    ProgressBar(ProgressBarBlock),
    
    /// Input field block
    Input(InputBlock),
}

/// Text block content
#[derive(Debug, Clone, PartialEq)]
pub struct TextBlock {
    /// Text content
    pub text: String,
    
    /// Text alignment
    pub alignment: TextAlignment,
    
    /// Font size multiplier
    pub font_size: f32,
    
    /// Whether text wraps
    pub wrap: bool,
    
    /// Maximum lines (0 = unlimited)
    pub max_lines: usize,
    
    /// Line height multiplier
    pub line_height: f32,
}

/// Button block content
#[derive(Debug, Clone, PartialEq)]
pub struct ButtonBlock {
    /// Button text
    pub text: String,
    
    /// Button state
    pub state: ButtonState,
    
    /// Button action
    pub action: String,
    
    /// Whether button is enabled
    pub enabled: bool,
    
    /// Button type
    pub button_type: ButtonType,
}

/// Image block content
#[derive(Debug, Clone, PartialEq)]
pub struct ImageBlock {
    /// Image source (path or URL)
    pub source: String,
    
    /// Image scaling mode
    pub scaling: ImageScaling,
    
    /// Alternative text
    pub alt_text: String,
    
    /// Whether image is loaded
    pub loaded: bool,
    
    /// Image dimensions (if loaded)
    pub dimensions: Option<(u32, u32)>,
}

/// Container block content
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerBlock {
    /// Child blocks
    pub children: Vec<String>,
    
    /// Layout direction
    pub direction: LayoutDirection,
    
    /// Spacing between children
    pub spacing: u16,
    
    /// Alignment of children
    pub alignment: Alignment,
    
    /// Whether to distribute space evenly
    pub distribute: bool,
}

/// Progress bar block content
#[derive(Debug, Clone, PartialEq)]
pub struct ProgressBarBlock {
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    
    /// Progress bar direction
    pub direction: ProgressDirection,
    
    /// Whether to show text
    pub show_text: bool,
    
    /// Custom text format
    pub text_format: Option<String>,
    
    /// Progress bar style
    pub bar_style: ProgressBarStyle,
}

/// Input field block content
#[derive(Debug, Clone, PartialEq)]
pub struct InputBlock {
    /// Current value
    pub value: String,
    
    /// Placeholder text
    pub placeholder: String,
    
    /// Input type
    pub input_type: InputType,
    
    /// Whether input is focused
    pub focused: bool,
    
    /// Maximum length
    pub max_length: Option<usize>,
    
    /// Cursor position
    pub cursor_position: usize,
}

/// Block position
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockPosition {
    /// X coordinate
    pub x: f32,
    
    /// Y coordinate
    pub y: f32,
    
    /// Position mode
    pub mode: PositionMode,
}

/// Block size
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockSize {
    /// Width
    pub width: f32,
    
    /// Height
    pub height: f32,
    
    /// Size mode
    pub mode: SizeMode,
}

/// Block styling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockStyle {
    /// Background color
    pub background: Option<Rgb>,
    
    /// Foreground color
    pub foreground: Option<Rgb>,
    
    /// Border color
    pub border: Option<Rgb>,
    
    /// Border width
    pub border_width: u16,
    
    /// Corner radius
    pub corner_radius: u16,
    
    /// Padding
    pub padding: Padding,
    
    /// Margin
    pub margin: Margin,
    
    /// Opacity (0.0 to 1.0)
    pub opacity: f32,
    
    /// Shadow
    pub shadow: Option<Shadow>,
}

/// Block configuration for serialization
#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq)]
pub struct BlockConfig {
    /// Block type
    pub block_type: String,
    
    /// Block position
    pub position: BlockPosition,
    
    /// Block size
    pub size: BlockSize,
    
    /// Block styling
    pub style: BlockStyle,
    
    /// Whether block is visible
    pub visible: bool,
    
    /// Z-order
    pub z_order: i32,
    
    /// Type-specific content
    pub content: BlockContent,
}

/// Block content for configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BlockContent {
    Text {
        text: String,
        alignment: TextAlignment,
        font_size: f32,
        wrap: bool,
        max_lines: usize,
        line_height: f32,
    },
    Button {
        text: String,
        action: String,
        enabled: bool,
        button_type: ButtonType,
    },
    Image {
        source: String,
        scaling: ImageScaling,
        alt_text: String,
    },
    Container {
        children: Vec<String>,
        direction: LayoutDirection,
        spacing: u16,
        alignment: Alignment,
        distribute: bool,
    },
    ProgressBar {
        progress: f32,
        direction: ProgressDirection,
        show_text: bool,
        text_format: Option<String>,
        bar_style: ProgressBarStyle,
    },
    Input {
        placeholder: String,
        input_type: InputType,
        max_length: Option<usize>,
    },
}

/// Block event handlers
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlockHandlers {
    /// Click handler
    pub on_click: Option<String>,
    
    /// Hover handler
    pub on_hover: Option<String>,
    
    /// Focus handler
    pub on_focus: Option<String>,
    
    /// Blur handler
    pub on_blur: Option<String>,
    
    /// Key press handler
    pub on_key_press: Option<String>,
    
    /// Value change handler (for inputs)
    pub on_change: Option<String>,
}

/// Text alignment
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Button state
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

/// Button type
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Link,
}

/// Image scaling mode
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ImageScaling {
    Fit,
    Fill,
    Stretch,
    Center,
    Tile,
}

/// Layout direction
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

/// Alignment
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

/// Progress direction
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ProgressDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

/// Progress bar style
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ProgressBarStyle {
    Solid,
    Striped,
    Animated,
}

/// Input type
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub enum InputType {
    Text,
    Password,
    Number,
    Email,
    Search,
}

/// Position mode
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PositionMode {
    /// Absolute position in pixels
    Absolute,
    /// Relative position as percentage
    Relative,
    /// Fixed position (doesn't scroll)
    Fixed,
}

/// Size mode
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SizeMode {
    /// Fixed size in pixels
    Fixed,
    /// Relative size as percentage
    Relative,
    /// Auto size based on content
    Auto,
    /// Fill available space
    Fill,
}

/// Padding
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Padding {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

/// Margin
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Margin {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

/// Shadow
#[derive(ConfigDeserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Shadow {
    pub color: Rgb,
    pub offset_x: i16,
    pub offset_y: i16,
    pub blur_radius: u16,
}

impl Default for BlockPosition {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            mode: PositionMode::Absolute,
        }
    }
}

impl Default for BlockSize {
    fn default() -> Self {
        Self {
            width: 100.0,
            height: 50.0,
            mode: SizeMode::Fixed,
        }
    }
}

impl Default for BlockStyle {
    fn default() -> Self {
        Self {
            background: None,
            foreground: None,
            border: None,
            border_width: 0,
            corner_radius: 0,
            padding: Padding { top: 4, right: 4, bottom: 4, left: 4 },
            margin: Margin { top: 0, right: 0, bottom: 0, left: 0 },
            opacity: 1.0,
            shadow: None,
        }
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }
}

impl Default for Margin {
    fn default() -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }
}

impl Default for Shadow {
    fn default() -> Self {
        Self {
            color: Rgb::new(0, 0, 0),
            offset_x: 0,
            offset_y: 0,
            blur_radius: 0,
        }
    }
}

// Add SerdeReplace implementations for the structs that need them
impl starterm_config::SerdeReplace for BlockPosition {
    fn replace(&mut self, value: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
        *self = serde::Deserialize::deserialize(value)?;
        Ok(())
    }
}

impl starterm_config::SerdeReplace for BlockSize {
    fn replace(&mut self, value: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
        *self = serde::Deserialize::deserialize(value)?;
        Ok(())
    }
}

impl starterm_config::SerdeReplace for BlockStyle {
    fn replace(&mut self, value: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
        *self = serde::Deserialize::deserialize(value)?;
        Ok(())
    }
}

impl starterm_config::SerdeReplace for BlockContent {
    fn replace(&mut self, value: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
        *self = serde::Deserialize::deserialize(value)?;
        Ok(())
    }
}

impl Default for TextAlignment {
    fn default() -> Self {
        Self::Left
    }
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Primary
    }
}

impl Default for ImageScaling {
    fn default() -> Self {
        Self::Fit
    }
}

impl Default for LayoutDirection {
    fn default() -> Self {
        Self::Vertical
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Start
    }
}

impl Default for ProgressDirection {
    fn default() -> Self {
        Self::LeftToRight
    }
}

impl Default for ProgressBarStyle {
    fn default() -> Self {
        Self::Solid
    }
}

impl Default for InputType {
    fn default() -> Self {
        Self::Text
    }
}

impl Default for BlockContent {
    fn default() -> Self {
        Self::Text {
            text: String::new(),
            alignment: TextAlignment::Left,
            font_size: 1.0,
            wrap: true,
            max_lines: 0,
            line_height: 1.2,
        }
    }
}

impl Default for BlockConfig {
    fn default() -> Self {
        Self {
            block_type: String::new(),
            position: BlockPosition::default(),
            size: BlockSize::default(),
            style: BlockStyle::default(),
            visible: true,
            z_order: 0,
            content: BlockContent::default(),
        }
    }
}

impl Block {
    /// Create a new block
    pub fn new(id: String, block_type: BlockType) -> Self {
        Self {
            id,
            block_type,
            position: BlockPosition::default(),
            size: BlockSize::default(),
            style: BlockStyle::default(),
            visible: true,
            needs_redraw: true,
            z_order: 0,
            handlers: BlockHandlers::default(),
        }
    }
    
    /// Create a text block
    pub fn text(id: String, text: String) -> Self {
        Self::new(id, BlockType::Text(TextBlock {
            text,
            alignment: TextAlignment::Left,
            font_size: 1.0,
            wrap: true,
            max_lines: 0,
            line_height: 1.2,
        }))
    }
    
    /// Create a button block
    pub fn button(id: String, text: String, action: String) -> Self {
        Self::new(id, BlockType::Button(ButtonBlock {
            text,
            state: ButtonState::Normal,
            action,
            enabled: true,
            button_type: ButtonType::Primary,
        }))
    }
    
    /// Create an image block
    pub fn image(id: String, source: String) -> Self {
        Self::new(id, BlockType::Image(ImageBlock {
            source,
            scaling: ImageScaling::Fit,
            alt_text: String::new(),
            loaded: false,
            dimensions: None,
        }))
    }
    
    /// Create a container block
    pub fn container(id: String) -> Self {
        Self::new(id, BlockType::Container(ContainerBlock {
            children: Vec::new(),
            direction: LayoutDirection::Vertical,
            spacing: 4,
            alignment: Alignment::Start,
            distribute: false,
        }))
    }
    
    /// Create a progress bar block
    pub fn progress_bar(id: String, progress: f32) -> Self {
        Self::new(id, BlockType::ProgressBar(ProgressBarBlock {
            progress: progress.clamp(0.0, 1.0),
            direction: ProgressDirection::LeftToRight,
            show_text: true,
            text_format: None,
            bar_style: ProgressBarStyle::Solid,
        }))
    }
    
    /// Create an input field block
    pub fn input(id: String, placeholder: String) -> Self {
        Self::new(id, BlockType::Input(InputBlock {
            value: String::new(),
            placeholder,
            input_type: InputType::Text,
            focused: false,
            max_length: None,
            cursor_position: 0,
        }))
    }
    
    /// Check if the block needs to be redrawn
    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }
    
    /// Mark the block as redrawn
    pub fn mark_redrawn(&mut self) {
        self.needs_redraw = false;
    }
    
    /// Mark the block as needing redraw
    pub fn mark_dirty(&mut self) {
        self.needs_redraw = true;
    }
    
    /// Set block position
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
        self.mark_dirty();
    }
    
    /// Set block size
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.size.width = width;
        self.size.height = height;
        self.mark_dirty();
    }
    
    /// Set block visibility
    pub fn set_visible(&mut self, visible: bool) {
        if self.visible != visible {
            self.visible = visible;
            self.mark_dirty();
        }
    }
    
    /// Apply theme to block style
    pub fn apply_theme(&mut self, theme: &UiTheme) {
        if self.style.background.is_none() {
            self.style.background = Some(theme.background);
        }
        if self.style.foreground.is_none() {
            self.style.foreground = Some(theme.foreground);
        }
        if self.style.border.is_none() {
            self.style.border = Some(theme.border);
        }
        if self.style.border_width == 0 {
            self.style.border_width = theme.border_width;
        }
        if self.style.corner_radius == 0 {
            self.style.corner_radius = theme.corner_radius;
        }
        
        // Apply padding from theme if not set
        if self.style.padding.top == 0 && self.style.padding.right == 0 && 
           self.style.padding.bottom == 0 && self.style.padding.left == 0 {
            self.style.padding = Padding {
                top: theme.padding,
                right: theme.padding,
                bottom: theme.padding,
                left: theme.padding,
            };
        }
        
        self.mark_dirty();
    }
    
    /// Get block bounds
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.position.x, self.position.y, self.size.width, self.size.height)
    }
    
    /// Check if point is inside block
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        let (bx, by, bw, bh) = self.bounds();
        x >= bx && x <= bx + bw && y >= by && y <= by + bh
    }
}

impl From<BlockConfig> for Block {
    fn from(config: BlockConfig) -> Self {
        let block_type = match config.content {
            BlockContent::Text { text, alignment, font_size, wrap, max_lines, line_height } => {
                BlockType::Text(TextBlock {
                    text,
                    alignment,
                    font_size,
                    wrap,
                    max_lines,
                    line_height,
                })
            },
            BlockContent::Button { text, action, enabled, button_type } => {
                BlockType::Button(ButtonBlock {
                    text,
                    state: ButtonState::Normal,
                    action,
                    enabled,
                    button_type,
                })
            },
            BlockContent::Image { source, scaling, alt_text } => {
                BlockType::Image(ImageBlock {
                    source,
                    scaling,
                    alt_text,
                    loaded: false,
                    dimensions: None,
                })
            },
            BlockContent::Container { children, direction, spacing, alignment, distribute } => {
                BlockType::Container(ContainerBlock {
                    children,
                    direction,
                    spacing,
                    alignment,
                    distribute,
                })
            },
            BlockContent::ProgressBar { progress, direction, show_text, text_format, bar_style } => {
                BlockType::ProgressBar(ProgressBarBlock {
                    progress,
                    direction,
                    show_text,
                    text_format,
                    bar_style,
                })
            },
            BlockContent::Input { placeholder, input_type, max_length } => {
                BlockType::Input(InputBlock {
                    value: String::new(),
                    placeholder,
                    input_type,
                    focused: false,
                    max_length,
                    cursor_position: 0,
                })
            },
        };
        
        Self {
            id: format!("block_{}", config.block_type),
            block_type,
            position: config.position,
            size: config.size,
            style: config.style,
            visible: config.visible,
            needs_redraw: true,
            z_order: config.z_order,
            handlers: BlockHandlers::default(),
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block({}): {:?} at ({}, {})", 
               self.id, self.block_type, self.position.x, self.position.y)
    }
}
