pub mod utils;
pub mod events;
pub mod model;
pub mod terminal;
pub mod ui;
pub mod ssh_commands;

use color_eyre::eyre::Result;
use events::EventHandler;
use model::Model;
use terminal::{end_terminal, setup_terminal};
use ui::draw;

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;

    let mut model = Model::default();
    // Initial app draw
    draw(&mut terminal, &model)?;

    let event_handler = EventHandler::default();

    while !model.is_app_done() {
        if let Some(message) = event_handler.poll_message(&model)? {
            model.update(message);
            draw(&mut terminal, &model)?
        }
    }

    end_terminal(terminal)
}
