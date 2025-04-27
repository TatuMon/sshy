use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::ui::{
    color_variants::ColorVariant,
    ui_utils::{centered_rect_for_paragraph, popups},
};

pub fn draw_popup_with_cfg(f: &mut Frame, content: String, variant: ColorVariant) {
    let popup_block = popups::loading_popup_block();

    let color = match variant {
        ColorVariant::Success => Color::Green,
        ColorVariant::Danger => Color::Red
    };

    let styled_text = Text::styled(content, Style::default().fg(color));

    let paragraph = Paragraph::new(styled_text).block(popup_block);

    let area = centered_rect_for_paragraph(&paragraph, 50, 50, f.area());

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
