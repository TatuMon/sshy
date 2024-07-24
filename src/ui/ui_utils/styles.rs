use ratatui::style::{Style, Color};

pub fn highlighted() -> Style {
    Style::new()
        .bg(Color::LightYellow)
        .fg(Color::Black)
}
