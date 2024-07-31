use ratatui::{widgets::{Clear, Block, BorderType, Borders, Paragraph}, Frame};

use crate::{model::Model, ui::ui_utils::centered_rect};

pub fn draw_add_pub_key_popup(f: &mut Frame, model: &Model) {
    let model_json = serde_json::to_string_pretty(model)
        .unwrap_or(String::from("failed to serialize model for debugging"));

    let block = Block::default()
        .title("New public key")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let paragraph = Paragraph::new(model_json)
        .block(block);

    let area = centered_rect(50, 50, f.size());

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
