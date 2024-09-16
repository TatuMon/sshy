use color_eyre::owo_colors::OwoColorize;
use ratatui::style::{Color, Style};

pub fn highlighted_item() -> Style {
    Style::new().bg(Color::LightYellow).fg(Color::Black)
}

pub fn focused_border() -> Style {
    Style::new().fg(Color::LightYellow)
}

pub fn fg_danger() -> Style {
    Style::new().fg(Color::Red)
}

/// Creates a Style that set the foreground's color to blue, indicating that the
/// target is loading
pub fn loading_border() -> Style {
    Style::new().fg(Color::Blue)
}
