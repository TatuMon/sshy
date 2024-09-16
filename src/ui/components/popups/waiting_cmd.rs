use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::{commands::CmdTask, model::Model, ui::ui_utils::{centered_rect_for_paragraph, popups}};

pub fn draw_waiting_cmd(f: &mut Frame, model: &Model) {
    let cmd_task = model.get_current_command().unwrap();

    let popup_block = popups::loading_popup_block();

    let text = match cmd_task {
        CmdTask::SshKeygen => "Generating key..."
    };
    let styled_text = Text::styled(
        text,
        Style::default().fg(Color::Blue),
    );

    let paragraph = Paragraph::new(styled_text).block(popup_block);

    let area = centered_rect_for_paragraph(&paragraph, 50, 50, f.size());

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
