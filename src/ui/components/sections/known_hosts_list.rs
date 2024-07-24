use ratatui::{Frame, widgets::{Block, Borders, BorderType, List, ListDirection, ListItem, ListState}, style::{Style, Stylize, Color}, layout::{Rect, Layout, Direction, Constraint}};

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
        .highlight_style(styles::highlighted())
        .block(block);

    let mut list_state = ListState::default();

    let area = get_area(f.size());
    f.render_stateful_widget(list, area, &mut list_state);
}

fn get_area(frame_rect: Rect) -> Rect {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25)])
        .split(frame_rect)[0]
}
