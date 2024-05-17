use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal}, widgets::{Block, Borders, Paragraph, Tabs}
};
use std::{
    io::{stdout, Result}, thread::sleep, time::Duration
};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // TODO main loop
    let mut should_end = false;
    loop {
        if should_end {
            terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("BYE"),
                    area,
                );
            })?;
            sleep(Duration::from_secs(1));
            break;
        } else {
            terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Tabs::new(["hola", "si", "chau"]),
                    area,
                );
            })?;
        }

        if event::poll(Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    should_end = true;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

