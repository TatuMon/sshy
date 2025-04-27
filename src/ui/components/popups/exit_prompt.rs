
use ratatui::{
    layout::Alignment, text::Text, widgets::{block::Position, Block, BorderType, Borders, Clear, Paragraph}, Frame
};

use crate::ui::ui_utils::{centered_rect_for_paragraph, styles};

pub fn draw_exit_popup(f: &mut Frame) {
    let popup_block = Block::default()
        .title_alignment(Alignment::Center)
        .title_position(Position::Bottom)
        .title("q to confirm")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(styles::fg_danger());

    let exit_text = Text::styled(
        "Are you sure you want to quit?",
        styles::fg_danger(),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text).centered().block(popup_block);

    let area = centered_rect_for_paragraph(&exit_paragraph, 50, 50, f.area());

    f.render_widget(Clear, area);
    f.render_widget(exit_paragraph, area);
}
