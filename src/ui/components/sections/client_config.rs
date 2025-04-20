use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders},
    text::Line,
    Frame,
};
use tui_textarea::{TextArea, CursorMove};

use crate::{
    model::sections_state::client_config_state::ClientConfigState , ui::ui_utils::styles,
};

/// USAR EL EJEMPLO DE VIM DE ACA
/// https://github.com/rhysd/tui-textarea/blob/main/examples/vim.rs

pub fn draw<'a>(f: &mut Frame, section_state: &'a ClientConfigState) {
    let mut block = Block::default()
        .title("SSH Client Config")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let is_user_interacting = section_state.is_interactive_on();

    if !is_user_interacting {
        block = block.title(Line::from("Press ⏎ to enter interactive mode").right_aligned());
    } else {
        block = block.title(Line::from("Press q to enter interactive mode").right_aligned());
    }

    if section_state.has_focus() {
        block = block.border_style(styles::textarea_border(is_user_interacting));
    }

    let textarea_state = section_state.get_textarea_state();
    let mut textarea = TextArea::from(textarea_state.get_content().lines());
    textarea.set_block(block);

    let cursor_pos = textarea_state.get_cursor_pos();
    textarea.move_cursor(CursorMove::Jump(cursor_pos.0, cursor_pos.1));

    let area = get_area(f.area());

    f.render_widget(&textarea, area);
}

fn get_area(frame_rect: Rect) -> Rect {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(66)])
        .split(frame_rect)[1]
}
