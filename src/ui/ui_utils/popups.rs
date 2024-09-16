use ratatui::widgets::{Block, BorderType, Borders};

use super::styles;

/// Creates a popup block with the given title and rounded borders
pub fn basic_popup_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(styles::focused_border())
}

/// Creates a popup block with the given title and blue rounded borders
pub fn loading_popup_block<'a>() -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(styles::loading_border())
}
