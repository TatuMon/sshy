pub mod messages;

use std::time::Duration;

use color_eyre::eyre::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{
    model::Model,
    ui::{components::popups::Popup, Focus},
};

use self::messages::Message;

/// Returns a message based on the incoming event
///
/// This event could be user input, incoming data of the SSH session, etc.
///
/// # NOTE
/// This function is non-blocking
pub fn poll_message(model: &Model) -> Result<Option<Message>> {
    if let Some(event) = poll_event()? {
        match event {
            Event::Key(key_event) => Ok(handle_key_event(key_event, model)),
            Event::Resize(_, _) => Ok(Some(Message::Draw)),
            // Event::Mouse(mouse_event) => Ok(handle_mouse_event(mouse_event)),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

/// Check if there's an unhandled event waiting to be handled
///
/// # NOTE
/// This function is non-blocking
fn poll_event() -> Result<Option<Event>> {
    if event::poll(Duration::from_secs(0)).wrap_err("Error polling for an event")? {
        Ok(Some(event::read().wrap_err("Error reading event")?))
    } else {
        Ok(None)
    }
}

/// Handles the key event and returns the appropiate message, based on the given
/// key's code and the model's state
///
/// # NOTE
/// When this function starts to get too complex, try to give each key code a
/// separate function
fn handle_key_event(event: KeyEvent, model: &Model) -> Option<Message> {
    match event.code {
        KeyCode::Char('q') => match model.get_focus() {
            Focus::Section(_) => Some(Message::ShowPopup(Popup::ExitPrompt)),
            Focus::Popup(popup) => match popup {
                Popup::ExitPrompt => Some(Message::StopApp),
                _ => None
            }
        },
        KeyCode::Char('p') => Some(Message::ShowPopup(Popup::DebugModel)),
        KeyCode::Esc => match model.on_popup() {
            true => Some(Message::HidePopup),
            false => None,
        },
        _ => None,
    }
}
