//! Handler to take user input as Vim motions
//!
//! Based on this example [here](https://github.com/rhysd/tui-textarea/blob/main/examples/vim.rs)

use std::fmt;
use tui_textarea::Input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
}

impl fmt::Display for VimMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Visual => write!(f, "VISUAL"),
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
    pub fn get_mode(&self) -> VimMode {
        self.mode
    }

    pub fn get_pending_input(&self) -> Input {
        self.pending.clone()
    }

    pub fn set_mode(&mut self, mode: VimMode) {
        self.mode = mode;
    }
}
