use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders, List, ListDirection, ListState},
    Frame,
};

use crate::{
    model::sections_state::known_hosts_list_state::KnownHostsListState, ui::ui_utils::styles,
};

pub fn draw(f: &mut Frame, rect: &Rect, section_state: &KnownHostsListState) {
    let mut block = Block::default()
        .title("Known hosts")
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

    let mut list_state = ListState::default();
    list_state.select(section_state.get_selected_item_idx());

    f.render_stateful_widget(list, *rect, &mut list_state);
}
