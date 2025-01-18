use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders};

use super::styles;

pub fn rounded_block(title: Option<&str>, border_style: Option<Style>) -> Block {
    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    if let Some(title) = title {
        block = block.title(title);
    }
    if let Some(style) = border_style {
        block = block.border_style(style);
    }

    block
}

/// Creates a popup block with the given title and rounded borders
pub fn basic_popup_block(title: &str) -> Block {
    rounded_block(Some(title), Some(styles::focused_border()))
}

/// Creates a popup block with the given title and blue rounded borders
pub fn loading_popup_block<'a>() -> Block<'a> {
    rounded_block(None, Some(styles::loading_border()))
}

pub fn error_popup_block<'a>() -> Block<'a> {
    rounded_block(None, Some(styles::fg_danger()))
}

pub fn warning_popup_block<'a>() -> Block<'a> {
    rounded_block(None, Some(styles::fg_warning()))
}
