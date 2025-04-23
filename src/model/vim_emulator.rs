//! Handler to take user input as Vim motions
//!
//! Based on this example [here](https://github.com/rhysd/tui-textarea/blob/main/examples/vim.rs)

use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};
use std::fmt;
use tui_textarea::Input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
    Operator(char),
}

impl VimMode {
    /// Returns a ratatui block widget with the title depending on the mode
    fn block<'a>(&self) -> Block<'a> {
        let help = match self {
            Self::Normal => "type q to quit, type i to enter insert mode",
            Self::Insert => "type Esc to back to normal mode",
            Self::Visual => "type y to yank, type d to delete, type Esc to back to normal mode",
            Self::Operator(_) => "move cursor to apply operator",
        };
        let title = format!("{} MODE ({})", self, help);
        Block::default().borders(Borders::ALL).title(title)
    }

    /// Returns a Style based on the mode
    fn cursor_style(&self) -> Style {
        let color = match self {
            Self::Normal => Color::Reset,
            Self::Insert => Color::LightBlue,
            Self::Visual => Color::LightYellow,
            Self::Operator(_) => Color::LightGreen,
        };
        Style::default().fg(color).add_modifier(Modifier::REVERSED)
    }
}

impl fmt::Display for VimMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Visual => write!(f, "VISUAL"),
            Self::Operator(c) => write!(f, "OPERATOR({})", c),
        }
    }
}

// State of Vim emulation
#[derive(Clone)]
pub struct VimState {
    mode: VimMode,
    pending: Input, // Pending input to handle a sequence with two keys like gg
}

impl Default for VimState {
    fn default() -> Self {
        Self {
            mode: VimMode::Normal,
            pending: Input::default() // Null
        }
    }
}

impl VimState {
    pub fn new(mode: VimMode) -> Self {
        Self {
            mode,
            pending: Input::default(), // Null
        }
    }

    pub fn get_mode(&self) -> VimMode {
        self.mode
    }

    pub fn get_pending_input(&self) -> Input {
        self.pending.clone()
    }

    pub fn set_mode(&mut self, mode: VimMode) {
        self.mode = mode;
    }

    fn with_pending(self, pending: Input) -> Self {
        Self {
            mode: self.mode,
            pending,
        }
    }
}
