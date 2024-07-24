use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{block::Position, Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::ui::ui_utils::centered_rect_for_paragraph;

pub fn draw_exit_popup(f: &mut Frame) {
    // f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
    let popup_block = Block::default()
        .title_alignment(Alignment::Center)
        .title_position(Position::Bottom)
        .title("q to confirm")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let exit_text = Text::styled(
        "Are you sure you want to exit the application?",
        Style::default().fg(Color::Red),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text).block(popup_block);

    let area = centered_rect_for_paragraph(&exit_paragraph, 50, 50, f.size());
    f.render_widget(exit_paragraph, area);
}
