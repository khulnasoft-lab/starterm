use std::collections::HashMap;

use crossfont::Metrics;
use starterm_terminal::index::Point;

use crate::display::color::Rgb;

use crate::display::SizeInfo;
use crate::renderer::{GlyphCache, Renderer};
use crate::renderer::rects::RenderRect;
use crate::ui::block::{
    Block, BlockType, TextBlock, ButtonBlock, ImageBlock, ContainerBlock, 
    ProgressBarBlock, InputBlock, ButtonState, TextAlignment, ProgressDirection,
    InputType
};
use crate::ui::UiTheme;

/// Render context for UI blocks
pub struct RenderContext<'a> {
    pub renderer: &'a mut Renderer,
    pub glyph_cache: &'a mut GlyphCache,
    pub size_info: &'a SizeInfo,
    pub theme: &'a UiTheme,
    pub metrics: &'a Metrics,
}

/// Block renderer for UI blocks
pub struct BlockRenderer;

impl BlockRenderer {
    /// Render a single block
    pub fn render_block(
        block: &Block,
        context: &mut RenderContext<'_>,
    ) {
        if !block.visible {
            return;
        }

        // Render background and border first
        Self::render_background(block, context);
        Self::render_border(block, context);

        // Render content based on block type
        match &block.block_type {
            BlockType::Text(text_block) => {
                Self::render_text_block(block, text_block, context);
            },
            BlockType::Button(button_block) => {
                Self::render_button_block(block, button_block, context);
            },
            BlockType::Image(image_block) => {
                Self::render_image_block(block, image_block, context);
            },
            BlockType::Container(container_block) => {
                Self::render_container_block(block, container_block, context);
            },
            BlockType::ProgressBar(progress_block) => {
                Self::render_progress_bar_block(block, progress_block, context);
            },
            BlockType::Input(input_block) => {
                Self::render_input_block(block, input_block, context);
            },
        }
    }

    /// Render all blocks in Z-order
    pub fn render_blocks(
        blocks: &HashMap<String, Block>,
        context: &mut RenderContext<'_>,
    ) {
        let mut sorted_blocks: Vec<_> = blocks.values().collect();
        sorted_blocks.sort_by_key(|block| block.z_order);

        for block in sorted_blocks {
            Self::render_block(block, context);
        }
    }

    /// Render block background
    fn render_background(block: &Block, _context: &mut RenderContext<'_>) {
        if let Some(_bg_color) = block.style.background {
            // TODO: Implement background rendering
            // For now, this is a placeholder
        }
    }

    /// Render block border
    fn render_border(block: &Block, _context: &mut RenderContext<'_>) {
        if let Some(border_color) = block.style.border {
            let _border_width = block.style.border_width as f32;
            let _rect = RenderRect {
                x: block.position.x,
                y: block.position.y,
                width: block.size.width,
                height: block.size.height,
                color: border_color,
                alpha: block.style.opacity,
                kind: crate::renderer::rects::RectKind::Normal,
            };
            
            // TODO: Implement border rendering
            // For now, this is a placeholder
        }
    }

    /// Render text block
    fn render_text_block(
        block: &Block,
        text_block: &TextBlock,
        context: &mut RenderContext<'_>,
    ) {
        let fg_color = block.style.foreground.unwrap_or(context.theme.foreground);
        let bg_color = block.style.background.unwrap_or(context.theme.background);

        // Calculate text area (excluding padding)
        let text_x = block.position.x + block.style.padding.left as f32;
        let text_y = block.position.y + block.style.padding.top as f32;
        let text_width = block.size.width - (block.style.padding.left + block.style.padding.right) as f32;
        let text_height = block.size.height - (block.style.padding.top + block.style.padding.bottom) as f32;

        // Split text into lines
        let lines = Self::wrap_text(&text_block.text, text_width, text_block.wrap, context);
        let max_lines = if text_block.max_lines > 0 {
            text_block.max_lines
        } else {
            lines.len()
        };

        let line_height = context.size_info.cell_height() * text_block.line_height;
        let total_text_height = (max_lines as f32 * line_height).min(text_height);

        // Calculate starting Y position based on alignment
        let mut current_y = match text_block.alignment {
            TextAlignment::Left | TextAlignment::Center | TextAlignment::Right => {
                text_y + (text_height - total_text_height) / 2.0
            },
            TextAlignment::Justify => text_y,
        };

        // Render each line
        for (_i, line) in lines.iter().enumerate().take(max_lines) {
            if current_y + line_height > text_y + text_height {
                break;
            }

            let line_x = match text_block.alignment {
                TextAlignment::Left | TextAlignment::Justify => text_x,
                TextAlignment::Center => {
                    text_x + (text_width - Self::calculate_text_width(line, context)) / 2.0
                },
                TextAlignment::Right => {
                    text_x + text_width - Self::calculate_text_width(line, context)
                },
            };

            // Convert to terminal coordinates
            let term_x = (line_x / context.size_info.cell_width()) as usize;
            let term_y = (current_y / context.size_info.cell_height()) as usize;

            context.renderer.draw_string(
                Point::new(term_y, starterm_terminal::index::Column(term_x)),
                fg_color,
                bg_color,
                line.chars(),
                context.size_info,
                context.glyph_cache,
            );

            current_y += line_height;
        }
    }

    /// Render button block
    fn render_button_block(
        block: &Block,
        button_block: &ButtonBlock,
        context: &mut RenderContext<'_>,
    ) {
        // Get button colors based on state
        let (fg_color, bg_color) = Self::get_button_colors(button_block, block, context.theme);

        // Render button background
        let button_rect = RenderRect {
            x: block.position.x,
            y: block.position.y,
            width: block.size.width,
            height: block.size.height,
            color: bg_color,
            alpha: 1.0,
            kind: crate::renderer::rects::RectKind::Normal,
        };
        context.renderer.draw_rects(context.size_info, context.metrics, vec![button_rect]);

        // Render button text (centered)
        let text_x = block.position.x + (block.size.width - Self::calculate_text_width(&button_block.text, context)) / 2.0;
        let text_y = block.position.y + (block.size.height - context.size_info.cell_height()) / 2.0;

        let term_x = (text_x / context.size_info.cell_width()) as usize;
        let term_y = (text_y / context.size_info.cell_height()) as usize;

        context.renderer.draw_string(
            Point::new(term_y, starterm_terminal::index::Column(term_x)),
            fg_color,
            bg_color,
            button_block.text.chars(),
            context.size_info,
            context.glyph_cache,
        );
    }

    /// Render image block
    fn render_image_block(
        block: &Block,
        image_block: &ImageBlock,
        context: &mut RenderContext<'_>,
    ) {
        // For now, render a placeholder with the alt text
        let fg_color = block.style.foreground.unwrap_or(context.theme.foreground);
        let bg_color = block.style.background.unwrap_or(context.theme.background);

        let placeholder_text = if image_block.alt_text.is_empty() {
            "[Image]"
        } else {
            &image_block.alt_text
        };

        let text_x = block.position.x + (block.size.width - Self::calculate_text_width(placeholder_text, context)) / 2.0;
        let text_y = block.position.y + (block.size.height - context.size_info.cell_height()) / 2.0;

        let term_x = (text_x / context.size_info.cell_width()) as usize;
        let term_y = (text_y / context.size_info.cell_height()) as usize;

        context.renderer.draw_string(
            Point::new(term_y, starterm_terminal::index::Column(term_x)),
            fg_color,
            bg_color,
            placeholder_text.chars(),
            context.size_info,
            context.glyph_cache,
        );
    }

    /// Render container block
    fn render_container_block(
        _block: &Block,
        _container_block: &ContainerBlock,
        _context: &mut RenderContext<'_>,
    ) {
        // Container blocks don't render content themselves
        // Their children are rendered separately
    }

    /// Render progress bar block
    fn render_progress_bar_block(
        block: &Block,
        progress_block: &ProgressBarBlock,
        context: &mut RenderContext<'_>,
    ) {
        let bg_color = block.style.background.unwrap_or(context.theme.background);
        let fg_color = block.style.foreground.unwrap_or(context.theme.accent);

        // Calculate progress dimensions
        let progress_width = match progress_block.direction {
            ProgressDirection::LeftToRight | ProgressDirection::RightToLeft => {
                block.size.width * progress_block.progress
            },
            ProgressDirection::TopToBottom | ProgressDirection::BottomToTop => {
                block.size.width
            },
        };

        let progress_height = match progress_block.direction {
            ProgressDirection::LeftToRight | ProgressDirection::RightToLeft => {
                block.size.height
            },
            ProgressDirection::TopToBottom | ProgressDirection::BottomToTop => {
                block.size.height * progress_block.progress
            },
        };

        // Calculate progress position
        let (progress_x, progress_y) = match progress_block.direction {
            ProgressDirection::LeftToRight | ProgressDirection::TopToBottom => {
                (block.position.x, block.position.y)
            },
            ProgressDirection::RightToLeft => {
                (block.position.x + block.size.width - progress_width, block.position.y)
            },
            ProgressDirection::BottomToTop => {
                (block.position.x, block.position.y + block.size.height - progress_height)
            },
        };

        // Render progress bar
        let progress_rect = RenderRect {
            x: progress_x,
            y: progress_y,
            width: progress_width,
            height: progress_height,
            color: fg_color,
            alpha: 1.0,
            kind: crate::renderer::rects::RectKind::Normal,
        };

        context.renderer.draw_rects(context.size_info, context.metrics, vec![progress_rect]);

        // Render progress text if enabled
        if progress_block.show_text {
            let text = if let Some(format) = &progress_block.text_format {
                format.replace("{progress}", &format!("{:.0}%", progress_block.progress * 100.0))
            } else {
                format!("{:.0}%", progress_block.progress * 100.0)
            };

            let text_x = block.position.x + (block.size.width - Self::calculate_text_width(&text, context)) / 2.0;
            let text_y = block.position.y + (block.size.height - context.size_info.cell_height()) / 2.0;

            let term_x = (text_x / context.size_info.cell_width()) as usize;
            let term_y = (text_y / context.size_info.cell_height()) as usize;

            context.renderer.draw_string(
                Point::new(term_y, starterm_terminal::index::Column(term_x)),
                context.theme.foreground,
                bg_color,
                text.chars(),
                context.size_info,
                context.glyph_cache,
            );
        }
    }

    /// Render input block
    fn render_input_block(
        block: &Block,
        input_block: &InputBlock,
        context: &mut RenderContext<'_>,
    ) {
        let fg_color = block.style.foreground.unwrap_or(context.theme.foreground);
        let bg_color = block.style.background.unwrap_or(context.theme.background);

        // Render input background
        let input_rect = RenderRect {
            x: block.position.x,
            y: block.position.y,
            width: block.size.width,
            height: block.size.height,
            color: bg_color,
            alpha: 1.0,
            kind: crate::renderer::rects::RectKind::Normal,
        };
        context.renderer.draw_rects(context.size_info, context.metrics, vec![input_rect]);

        // Render input text or placeholder
        let display_text = if input_block.value.is_empty() {
            &input_block.placeholder
        } else {
            &input_block.value
        };

        // Handle password input
        let rendered_text = if input_block.input_type == InputType::Password && !input_block.value.is_empty() {
            "*".repeat(input_block.value.len())
        } else {
            display_text.to_string()
        };

        let text_x = block.position.x + block.style.padding.left as f32;
        let text_y = block.position.y + (block.size.height - context.size_info.cell_height()) / 2.0;

        let term_x = (text_x / context.size_info.cell_width()) as usize;
        let term_y = (text_y / context.size_info.cell_height()) as usize;

        let text_color = if input_block.value.is_empty() {
            // Use a dimmed color for placeholder
            Rgb::new(
                fg_color.r / 2,
                fg_color.g / 2,
                fg_color.b / 2,
            )
        } else {
            fg_color
        };

        context.renderer.draw_string(
            Point::new(term_y, starterm_terminal::index::Column(term_x)),
            text_color,
            bg_color,
            rendered_text.chars(),
            context.size_info,
            context.glyph_cache,
        );

        // Render cursor if focused
        if input_block.focused {
            let cursor_x = text_x + (input_block.cursor_position as f32 * context.size_info.cell_width());
            let cursor_rect = RenderRect {
                x: cursor_x,
                y: text_y,
                width: 2.0,
                height: context.size_info.cell_height(),
                color: fg_color,
                alpha: 1.0,
                kind: crate::renderer::rects::RectKind::Normal,
            };
            context.renderer.draw_rects(context.size_info, context.metrics, vec![cursor_rect]);
        }
    }

    /// Get button colors based on state
    fn get_button_colors(
        button_block: &ButtonBlock,
        block: &Block,
        theme: &UiTheme,
    ) -> (Rgb, Rgb) {
        let base_fg = block.style.foreground.unwrap_or(theme.foreground);
        let base_bg = block.style.background.unwrap_or(theme.background);

        match button_block.state {
            ButtonState::Normal => {
                if button_block.enabled {
                    (base_fg, base_bg)
                } else {
                    (
                        Rgb::new(base_fg.r / 2, base_fg.g / 2, base_fg.b / 2),
                        Rgb::new(base_bg.r / 2, base_bg.g / 2, base_bg.b / 2),
                    )
                }
            },
            ButtonState::Hovered => {
                if button_block.enabled {
                    (base_fg, theme.accent)
                } else {
                    (
                        Rgb::new(base_fg.r / 2, base_fg.g / 2, base_fg.b / 2),
                        Rgb::new(base_bg.r / 2, base_bg.g / 2, base_bg.b / 2),
                    )
                }
            },
            ButtonState::Pressed => {
                if button_block.enabled {
                    (theme.background, theme.accent)
                } else {
                    (
                        Rgb::new(base_fg.r / 2, base_fg.g / 2, base_fg.b / 2),
                        Rgb::new(base_bg.r / 2, base_bg.g / 2, base_bg.b / 2),
                    )
                }
            },
            ButtonState::Disabled => {
                (
                    Rgb::new(base_fg.r / 2, base_fg.g / 2, base_fg.b / 2),
                    Rgb::new(base_bg.r / 2, base_bg.g / 2, base_bg.b / 2),
                )
            },
        }
    }

    /// Wrap text to fit within the specified width
    fn wrap_text(text: &str, width: f32, wrap: bool, context: &RenderContext<'_>) -> Vec<String> {
        if !wrap {
            return vec![text.to_string()];
        }

        let char_width = context.size_info.cell_width();
        let chars_per_line = (width / char_width) as usize;

        if chars_per_line == 0 {
            return vec![text.to_string()];
        }

        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 > chars_per_line {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                }
            }

            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    /// Calculate text width in pixels
    fn calculate_text_width(text: &str, context: &RenderContext<'_>) -> f32 {
        text.chars().count() as f32 * context.size_info.cell_width()
    }
}
