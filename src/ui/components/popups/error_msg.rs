use ratatui::{
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::{model::Model, ui::ui_utils::{centered_rect_for_paragraph, popups}};

pub fn draw_error_msg(f: &mut Frame, model: &Model) {
    let error_msg = model.get_current_error().unwrap();

    let popup_block = popups::error_popup_block();

    // let styled_text = Text::styled(
    //     error_msg,
    //     Style::default().fg(Color::Blue),
    // );

    let paragraph = Paragraph::new(error_msg).block(popup_block);

    let area = centered_rect_for_paragraph(&paragraph, 50, 50, f.area());

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
