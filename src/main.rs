pub mod utils;
pub mod events;
pub mod model;
pub mod terminal;
pub mod ui;

use color_eyre::eyre::Result;
use events::poll_message;
use model::Model;
use terminal::{end_terminal, setup_terminal};
use ui::draw;

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;

    let mut model = Model::default();
    // Initial app draw
    draw(&mut terminal, &model)?;

    while !model.is_app_done() {
        if let Some(message) = poll_message(&model)? {
            model.update(message);
            draw(&mut terminal, &model)?
        }
    }

    end_terminal(terminal)
}
