use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Clear},
    Frame,
};

use crate::{
    model::{sections_state::public_keys_list_state::NewPublicKeyFocus, Model},
    ui::ui_utils::{centered_rect, text_input},
};

pub fn draw_add_pub_key_popup(f: &mut Frame, model: &Model) {
    let new_key_state = model
        .get_sections_state()
        .get_public_keys_list_state()
        .get_new_key_state();

    let popup_block = Block::default()
        .title("New public key (ssh-keygen)")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let area = centered_rect(50, 50, f.size());

    f.render_widget(Clear, area);
    f.render_widget(popup_block, area);

    draw_key_name_input(
        f,
        area,
        new_key_state.get_name(),
        new_key_state.has_focus_on(NewPublicKeyFocus::Name),
    );
    draw_key_type(f, area, new_key_state.get_type().into());
    draw_key_comment_input(
        f,
        area,
        new_key_state.get_comment(),
        new_key_state.has_focus_on(NewPublicKeyFocus::Comment),
    );
}

fn draw_key_name_input(f: &mut Frame, popup_area: Rect, value: &str, focused: bool) {
    let input_paragraph = text_input::text_input("Filename (-f)", value, focused);

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

fn draw_key_type(f: &mut Frame, popup_area: Rect, value: &str) {
    let input_paragraph = text_input::text_input("Type (-t)", value, false);

    let top_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3)])
        .margin(2)
        .split(popup_area)[1];

    let input_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(top_area)[0];

    f.render_widget(input_paragraph, input_area);
}

fn draw_key_comment_input(f: &mut Frame, popup_area: Rect, value: &str, focused: bool) {
    let input_paragraph = text_input::text_input("Comment (-C)", value, focused);

    let top_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .margin(2)
        .split(popup_area)[2];

    let input_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(top_area)[0];

    f.render_widget(input_paragraph, input_area);
}
