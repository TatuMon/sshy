pub mod components;
pub mod ui_utils;

use color_eyre::eyre::{Context, Result};
use serde::Serialize;

use crate::{model::Model, terminal::SshyTerminal};

use self::{components::popups::{Popup, exit_prompt, debug_model}, components::sections::{Section, known_hosts_list}};

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

/// Main draw function
///
/// It'll internally draw the appropiate sections and popups based on the given
/// model's state
pub fn draw(terminal: &mut SshyTerminal, model: &Model) -> Result<()> {
    terminal
        .draw(|f| {
            known_hosts_list::draw(f);
            // We first draw the current section
            // And then the current popup (if any)
            if let Some(popup) = model.get_popup() {
                match popup {
                    Popup::ExitPrompt => exit_prompt::draw_exit_popup(f),
                    Popup::DebugModel => debug_model::draw_debug_model_popup(f, model),
                }
            }
        })
        .wrap_err("Drawing error")?;

    Ok(())
}
