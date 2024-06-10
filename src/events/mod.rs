pub mod bindings;
pub mod messages;

use std::time::Duration;

use color_eyre::eyre::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{
    model::Model,
    ui::{popups::Popup, Focus},
};

use self::{bindings::Q, messages::Message};

/// Return a message based on the incoming event
pub fn poll_message(model: &Model) -> Result<Option<Message>> {
    if let Some(event) = poll_event()? {
        match event {
            Event::Key(key_event) => Ok(handle_key_event(key_event, model)),
            // Event::Mouse(mouse_event) => Ok(handle_mouse_event(mouse_event)),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn poll_event() -> Result<Option<Event>> {
    if event::poll(Duration::from_secs(0)).wrap_err("Error polling for an event")? {
        Ok(Some(event::read().wrap_err("Error reading event")?))
    } else {
        Ok(None)
    }
}

fn handle_key_event(event: KeyEvent, model: &Model) -> Option<Message> {
    match event.code {
        KeyCode::Char(Q) => match model.get_focus() {
            Focus::Section(section) => match section {
                _ => Some(Message::ShowPopup(Popup::Exit)),
            },
            Focus::Popup(popup) => match popup {
                Popup::Exit => Some(Message::StopApp),
                _ => None
            }
        },
        KeyCode::Char(P) => Some(Message::ShowPopup(Popup::DebugModel)),
        KeyCode::Esc => match model.on_popup() {
            true => Some(Message::HidePopup),
            false => None,
        },
        _ => None,
    }
}
