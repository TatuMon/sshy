use ratatui::style::{Color, Style};

use crate::model::vim_emulator::{VimMode, VimState};

pub fn highlighted_item() -> Style {
    Style::new().bg(Color::LightYellow).fg(Color::Black)
}

pub fn focused_border() -> Style {
    Style::new().fg(Color::LightYellow)
}

pub fn fg_danger() -> Style {
    Style::new().fg(Color::Red)
}

pub fn fg_warning() -> Style {
    Style::new().fg(Color::Yellow)
}

/// Creates a Style that set the foreground's color to blue, indicating that the
/// target is loading
pub fn loading_border() -> Style {
    Style::new().fg(Color::Blue)
}

pub fn textarea_border(is_user_interacting: bool) -> Style {
    if is_user_interacting {
        Style::new().fg(Color::LightRed)
    } else {
        focused_border()
    }
}

pub fn vim_border(vim_state: &VimState) -> Option<Style> {
    match vim_state.get_mode() {
        VimMode::Normal => Some(Style::new().fg(Color::LightBlue)),
        VimMode::Insert => Some(Style::new().fg(Color::LightGreen)),
        VimMode::Visual => Some(Style::new().fg(Color::LightMagenta)),
        VimMode::Operator(_) => None
    }
}
