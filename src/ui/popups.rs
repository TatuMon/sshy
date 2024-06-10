use color_eyre::eyre::{Context, Result};
use ratatui::{widgets::{Block, Borders, BorderType, Paragraph, Wrap}, text::Text, style::{Color, Style}, Frame};
use serde::Serialize;

use crate::model::Model;

use super::centered_rect;

#[derive(Clone, Copy, Serialize)]
pub enum Popup {
    Exit,
    DebugModel
}

pub fn draw_exit_popup(f: &mut Frame, model: &Model) {
    // f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
    let popup_block = Block::default()
        .title("Y/N")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::DarkGray));

    let exit_text = Text::styled(
        "Would you like to output the buffer as json? (y/n)",
        Style::default().fg(Color::Red),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, f.size());
    f.render_widget(exit_paragraph, area);
}

pub fn draw_debug_model_popup(f: &mut Frame, model: &Model) -> Result<()> {
    let model_json = serde_json::to_string_pretty(model)
        .wrap_err("failed to serialize model for debugging")?;

    let block = Block::default()
        .title("Model state")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let paragraph = Paragraph::new(model_json)
        .block(block);

    let area = centered_rect(50, 50, f.size());
    Ok(f.render_widget(paragraph, area))
}
