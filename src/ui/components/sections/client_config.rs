use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders},
    text::Line,
    Frame,
};
use tui_textarea::TextArea;

use crate::{
    model::sections_state::client_config_state::ClientConfigState , ui::ui_utils::styles,
};

/// USAR EL EJEMPLO DE VIM DE ACA
/// https://github.com/rhysd/tui-textarea/blob/main/examples/vim.rs

pub fn draw(f: &mut Frame, section_state: &ClientConfigState) {
    let mut block = Block::default()
        .title("SSH Client Config")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    if section_state.is_user_modifying() {
        block = block.title(Line::from("Press"));
    }

    if section_state.has_focus() {
        block = block.border_style(styles::focused_border());
    }

    let content = section_state.get_content();

    let mut textarea = TextArea::from(content.to_owned().lines());
    textarea.set_block(block);

    let area = get_area(f.area());

    f.render_widget(&textarea, area);
}

fn get_area(frame_rect: Rect) -> Rect {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(66)])
        .split(frame_rect)[1]
}
