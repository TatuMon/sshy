use ratatui::{style::{Color, Style}, text::Text, widgets::{Block, BorderType, Borders, Paragraph, Wrap}, Frame};

use crate::{model::Model, ui::components::utils::centered_rect};


pub fn draw_exit_popup(f: &mut Frame, model: &Model) {
    // f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
    let popup_block = Block::default()
        .title("Q to confirm")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let exit_text = Text::styled(
        "Are you sure you want to exit the application?",
        Style::default().fg(Color::Red),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, f.size());
    f.render_widget(exit_paragraph, area);
}
