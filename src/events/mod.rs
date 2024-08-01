pub mod messages;

use std::time::Duration;

use color_eyre::eyre::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    model::Model,
    ui::{
        components::{popups::Popup, sections::Section},
        Focus,
    },
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
///
/// # TODO
/// Before matching the event's code, the function should first match the model's
/// focus
///
/// Matching the code first, makes it impossible to write to an input
fn handle_key_event(key: KeyEvent, model: &Model) -> Option<Message> {
    match model.get_focus() {
        Focus::Section(section) => handle_section_key_event(section, key),
        Focus::Popup(popup) => handle_popup_key_event(popup, key),
    }
}

fn handle_section_key_event(section: Section, event: KeyEvent) -> Option<Message> {
    match event.code {
        KeyCode::Char('q') => Some(Message::ShowPopup(Popup::ExitPrompt)),
        KeyCode::Char('p') => Some(Message::ShowPopup(Popup::DebugModel)),
        KeyCode::Right => Some(Message::MoveToNextSection),
        KeyCode::Left => Some(Message::MoveToPrevSection),
        KeyCode::Up => Some(Message::SelPrevListItem),
        KeyCode::Down => Some(Message::SelNextListItem),
        KeyCode::Char('n') => {
            if let Section::PublicKeysList = section {
                Some(Message::ShowPopup(Popup::AddPubKey))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn handle_popup_key_event(popup: Popup, key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char(ch) => match popup {
            Popup::ExitPrompt => {
                if ch == 'q' {
                    Some(Message::StopApp)
                } else {
                    None
                }
            }
            Popup::AddPubKey if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Message::PopWord)
            }
            Popup::AddPubKey => Some(Message::WriteChar(ch)),
            _ => None,
        },
        KeyCode::Backspace => Some(Message::PopChar),
        KeyCode::Esc => Some(Message::HidePopup),
        KeyCode::Tab => Some(Message::SelNextPopupItem),
        KeyCode::BackTab => Some(Message::SelPrevPopupItem),
        _ => None,
    }
}
