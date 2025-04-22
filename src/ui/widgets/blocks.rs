use ratatui::{style::Style, text::Line, widgets::{Block, BorderType, Borders}};

use crate::ui::ui_utils::styles;

pub fn ssh_config_block<'a>(
    is_interacting: bool,
    has_focus: bool,
) -> Block<'a> {
    let mut block = Block::default()
        .title("SSH Client Config")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    if has_focus {
        block = block.border_style(styles::textarea_border(is_interacting));
    }

    if !is_interacting {
        block = block.title(Line::from("Press ⏎ to enter interactive mode").right_aligned());
    } else {
        block = block.title(Line::from("Press q to enter interactive mode").right_aligned());
    }

    block
}
