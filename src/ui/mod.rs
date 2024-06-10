pub mod popups;
pub mod sections;

use color_eyre::eyre::{Context, Result};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, Frame, widgets::Paragraph};
use serde::Serialize;

use crate::{model::Model, terminal::SshyTerminal};

use self::{popups::{Popup, draw_exit_popup, draw_debug_model_popup}, sections::Section};

#[derive(Clone, Copy, Serialize)]
pub enum Focus {
    Popup(Popup),
    Section(Section),
}

impl Default for Focus {
    fn default() -> Self {
        Self::Section(Section::default())
    }
}

pub fn draw(terminal: &mut SshyTerminal, model: &Model) -> Result<()> {
    terminal
        .draw(|f| {
            // We first draw the current section
            match model.get_section() {
                Section::Home => draw_home(f, model),
            }
            // And then the current popup (if any)
            if let Some(popup) = model.get_popup() {
                match popup {
                    Popup::Exit => draw_exit_popup(f, model),
                    Popup::DebugModel => {draw_debug_model_popup(f, model);},
                }
            }
        })
        .wrap_err("Drawing error")?;

    Ok(())
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn draw_home(f: &mut Frame, model: &Model) {
    f.render_widget(Paragraph::new("Hello world!"), f.size());
}
