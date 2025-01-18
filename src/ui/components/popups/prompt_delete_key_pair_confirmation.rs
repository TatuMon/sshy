use ratatui::{
    text::Line,
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::model::Model;
use crate::ui::ui_utils::{centered_rect_for_paragraph, popups};

pub fn draw_prompt_delete_key_pair_confirmation(f: &mut Frame, model: &Model) {
    // This should panic if None because this draw MUST NOT be called if the conditions are not
    // met
    let target_key_pair = model
        .get_sections_state()
        .get_public_keys_list_state()
        .get_selected_key_name()
        .expect("must provide a key pair to delete");

    let popup_block = popups::warning_popup_block()
        .title_bottom(Line::from("Press ⏎ to confirm").right_aligned());

    let paragraph_content = format!("Delete '{}' key pair?", target_key_pair);
    let paragraph = Paragraph::new(paragraph_content).block(popup_block);

    let area = centered_rect_for_paragraph(&paragraph, 50, 50, f.size());

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
