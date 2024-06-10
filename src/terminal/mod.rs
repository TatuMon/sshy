use std::io::{self, Stdout};

use color_eyre::eyre::{Context, Result};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub type SshyTerminal = Terminal<CrosstermBackend<Stdout>>;

pub fn setup_terminal() -> Result<SshyTerminal> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).wrap_err("Error creating the terminal instance")
}

pub fn end_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .wrap_err("Error shutting down the terminal instance")
}
