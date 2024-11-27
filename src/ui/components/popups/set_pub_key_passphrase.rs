use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::Clear,
    Frame,
};

use crate::{
    model::{sections_state::public_keys_list_state::NewPublicKeyFocus, Model},
    ui::ui_utils::{centered_rect_px, popups, text_input},
};

pub fn draw_set_pub_key_passphrase(f: &mut Frame, model: &Model) {
    let new_key_state = model
        .get_sections_state()
        .get_public_keys_list_state()
        .get_new_key_state();

    let popup_block = popups::basic_popup_block("Enter passphrase (empty for no passphrase)")
        .title_bottom(Line::from("Press ⏎ to confirm").right_aligned());

    let f_area = f.size();
    let area = centered_rect_px(f_area.width / 2, 13, f_area);

    f.render_widget(Clear, area);
    f.render_widget(popup_block, area);

    draw_passphrase_input(
        f,
        area,
        new_key_state.get_passphrase_len(),
    );
}

fn draw_passphrase_input(f: &mut Frame, popup_area: Rect, passphrase_len: usize) {
    let input_value = "*".repeat(passphrase_len);

    let input_paragraph = text_input::text_input("Passphrase", &input_value, true);

    let top_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)])
        .margin(2)
        .split(popup_area)[0];

    let input_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(top_area)[0];

    f.render_widget(input_paragraph, input_area);
}
