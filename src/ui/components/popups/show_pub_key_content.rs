use ratatui::{
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::{
    model::Model,
    ui::ui_utils::{centered_rect_for_paragraph, popups, width_percentage_to_px},
};

pub fn draw_pub_key_content(f: &mut Frame, model: &Model) {
    let bind = model
        .get_sections_state()
        .get_public_keys_list_state()
        .get_selected_key_path().expect("failed to get key path");

    let key_path = bind.to_str();

    let pub_key_content = model
        .get_sections_state()
        .get_public_keys_list_state()
        .get_selected_key_content().expect("failed to get key content");

    let wrapped_content = textwrap::fill(
        &pub_key_content,
        width_percentage_to_px(f.area(), 50)
    );

    let popup_block = popups::rounded_block(key_path, None);

    let paragraph = Paragraph::new(wrapped_content).block(popup_block);

    let area = centered_rect_for_paragraph(&paragraph, 75, 75, f.area());

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
