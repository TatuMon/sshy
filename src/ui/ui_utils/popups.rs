use ratatui::widgets::{Block, BorderType, Borders};

use super::styles;

pub fn basic_popup_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(styles::focused_border())
}
