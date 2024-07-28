use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, List, ListDirection, ListState},
    Frame,
};

use crate::{ui::ui_utils::styles, model::sections_state::public_keys_list_state::PublicKeysListState};

pub fn draw(f: &mut Frame, section_state: &PublicKeysListState) {
    let mut block = Block::default()
        .title("Public keys")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    if section_state.has_focus() {
        block = block.border_style(styles::focused_border());
    }

    let list = List::default()
        .items(section_state.get_items())
        .direction(ListDirection::TopToBottom)
        .highlight_style(styles::highlighted_item())
        .block(block);

    let area = get_area(f.size());
    let mut list_state = ListState::default();
    list_state.select(section_state.get_selected_item_idx());

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
        .split(column)[1]
}
