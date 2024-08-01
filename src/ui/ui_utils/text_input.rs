use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use super::styles;

/// One-line height text input
pub fn text_input<'a>(label: &'a str, value: &'a str, focused: bool) -> Paragraph<'a> {
    let mut block = Block::default()
        .title(label)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    if focused {
        block = block.style(styles::focused_border());
    }

    let paragraph = Paragraph::new(value.to_owned()).block(block);

    paragraph.to_owned()
}
