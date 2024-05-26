use std::{
    io::{stdout, Result},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};

pub fn run() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut should_end = false;
    loop {
        if !should_end {
            terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("Hello world! Press Q to exit")
                        .white()
                        .on_light_blue(),
                    area,
                );
            })?;
        } else {
            terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(Paragraph::new("Bye!"), area);
            })?;
            sleep(Duration::from_secs(1));
            break;
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
