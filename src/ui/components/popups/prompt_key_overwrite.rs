use ratatui::{
    text::Line,
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::{
    model::Model,
    ui::ui_utils::{centered_rect_for_paragraph, popups},
};

pub fn draw_prompt_key_overwrite(f: &mut Frame, model: &Model) {
    let new_key_name = model
        .get_sections_state()
        .get_public_keys_list_state()
        .get_new_key_state()
        .get_name();

    let popup_block = popups::warning_popup_block()
        .title_bottom(Line::from("Press ⏎ to confirm").right_aligned());

    let paragraph_content = format!("The key '{}' already exists. Overwrite?", new_key_name);
    let paragraph = Paragraph::new(paragraph_content).block(popup_block);

    let area = centered_rect_for_paragraph(&paragraph, 50, 50, f.area());

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
