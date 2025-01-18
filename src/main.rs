mod commands;
mod events;
mod model;
mod terminal;
mod ui;
mod utils;
mod async_jobs;

use color_eyre::eyre::Result;
use events::EventHandler;
use model::Model;
use terminal::{end_terminal, setup_terminal};

#[tokio::main]
async fn main() -> Result<()> {
    terminal::init_panic_hook();
    let mut terminal = setup_terminal()?;

    let mut model = Model::default();
    // Initial app draw
    ui::draw(&mut terminal, &model)?;

    let mut event_handler = EventHandler::default();

    while !model.is_app_done() {
        for message in event_handler.poll_messages(&model)? {
            model.update(message);
            ui::draw(&mut terminal, &model)?
        }
    }

    end_terminal(terminal)
}
