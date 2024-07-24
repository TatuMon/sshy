use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, List, ListDirection, ListState},
    Frame,
};

use crate::{ui::ui_utils::styles, utils};

pub fn draw(f: &mut Frame) {
    let block = Block::default()
        .title("Known hosts")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let items = utils::files::get_known_hosts().unwrap();

    let list = List::default()
        .items(items)
        .direction(ListDirection::TopToBottom)
        .highlight_style(styles::highlighted_item())
        .block(block);

    let mut list_state = ListState::default();

    let area = get_area(f.size());
    f.render_stateful_widget(list, area, &mut list_state);
}

fn get_area(frame_rect: Rect) -> Rect {
    let column = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33)])
        .split(frame_rect)[0];

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33)])
        .split(column)[0]
}
