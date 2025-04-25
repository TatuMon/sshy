use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::model::sections_state::client_config_state::ClientConfigState;

/// USAR EL EJEMPLO DE VIM DE ACA
/// https://github.com/rhysd/tui-textarea/blob/main/examples/vim.rs

pub fn draw<'a>(f: &mut Frame, section_state: &'a ClientConfigState) {
    let textarea = section_state.get_textarea();
    let area = get_area(f.area());

    f.render_widget(textarea, area);
}

fn get_area(frame_rect: Rect) -> Rect {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(66)])
        .split(frame_rect)[1]
}
