use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, List, ListDirection, ListItem, ListState},
    Frame,
};

use crate::ui::ui_utils::styles;

pub fn draw(f: &mut Frame, items: Vec<ListItem>, list_state: &mut ListState, focused: bool) {
    let block = Block::default()
        .title("Public keys")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    if focused {
        block.border_style(styles::focused_border());
    }

    let list = List::default()
        .items(items)
        .direction(ListDirection::TopToBottom)
        .highlight_style(styles::highlighted_item())
        .block(block);

    f.render_stateful_widget(list, get_area(f.size()), list_state);
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
