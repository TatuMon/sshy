use ratatui::{style::Style, text::Line, widgets::{Block, BorderType, Borders}};

use crate::{model::vim_emulator::{VimMode, VimState}, ui::ui_utils::styles};

pub fn ssh_config_block<'a>(
    is_interacting: bool,
    has_focus: bool,
    vim_state: &VimState
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
        block = block.title(Line::from("Press q to exit interactive mode").right_aligned());
        block = block.title_bottom(Line::from(vim_state.get_mode().to_string()).left_aligned());

        if let Some(vim_border) = styles::vim_border(vim_state) {
            block = block.border_style(vim_border);
        }

        if vim_state.get_mode() == VimMode::Normal {
            block = block.title_bottom(Line::from("Press CTRL+s to write to buffer").right_aligned());
        }
    }

    block
}
