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
