use ratatui::{
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::{
    model::Model,
    ui::ui_utils::{centered_rect, popups},
};

pub fn draw_debug_model_popup(f: &mut Frame, model: &Model) {
    let model_json = serde_json::to_string_pretty(model)
        .unwrap_or(String::from("failed to serialize model for debugging"));

    let block = popups::basic_popup_block("Model state");

    let paragraph = Paragraph::new(model_json).block(block);

    let area = centered_rect(50, 50, f.area());

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
