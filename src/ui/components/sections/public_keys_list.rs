use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, List, ListState},
    Frame,
};

use crate::{ui::ui_utils::styles, utils};

pub fn draw(f: &mut Frame) {
    let block = Block::default()
        .title("Public keys")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let items = utils::files::get_public_keys_names().unwrap_or_default();

    let list = List::default()
        .items(items)
        .block(block)
        .highlight_style(styles::highlighted_item());

    let mut list_state = ListState::default();

    f.render_stateful_widget(list, get_area(f.size()), &mut list_state);
}

fn get_area(frame_rect: Rect) -> Rect {
    let column = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33)])
        .split(frame_rect)[0];

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33)])
        .split(column)[1]
}
