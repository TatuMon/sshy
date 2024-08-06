pub mod messages;

use std::{sync::mpsc, time::Duration};

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

pub struct EventHandler {
    msg_rx: mpsc::Receiver<Message>,
    msg_tx: mpsc::Sender<Message>,
}

impl Default for EventHandler {
    fn default() -> Self {
        let (msg_tx, msg_rx) = mpsc::channel::<Message>();

        Self { msg_rx, msg_tx }
    }
}

impl EventHandler {
    /// Returns a message based on the incoming event
    ///
    /// This event could be user input, incoming data of the SSH session, etc.
    ///
    /// # NOTE
    /// This function is non-blocking
    pub fn poll_message(&self, model: &Model) -> Result<Option<Message>> {
        if let Some(event) = self.poll_crossterm_event()? {
            match event {
                Event::Key(key_event) => Ok(self.handle_key_event(key_event, model)),
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
    fn poll_crossterm_event(&self) -> Result<Option<Event>> {
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
    fn handle_key_event(&self, key: KeyEvent, model: &Model) -> Option<Message> {
        match model.get_focus() {
            Focus::Section(section) => self.handle_section_key_event(section, key),
            Focus::Popup(popup) => self.handle_popup_key_event(popup, key),
        }
    }

    fn handle_section_key_event(&self, section: Section, event: KeyEvent) -> Option<Message> {
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

    fn handle_popup_key_event(&self, popup: Popup, key: KeyEvent) -> Option<Message> {
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
            KeyCode::Enter => {
                if let Popup::AddPubKey = popup {
                    //  - The ssh-keygen command will be ran here
                    //  - Create an "EventHandler" struct that will run these existing
                    //      functions but will algo have a mpsc channel
                    //  - THIS branch will call the ssh_commands::SshKeygenCmd::run method,
                    //      which will receive a "tx" from the EventHandler's mpsc channel
                    //  - The keygen command will produce the appropiate messages based
                    //      on the command's outputs (like prompting the user for a
                    //      passphare) and send them through the "tx"
                    //  - The method EventHandler::poll_chan_events will read the associated
                    //      mpsc channel and handle the messages appropiately
                    //      - In other words: the ssh-keygen command struct will PRODUCE
                    //          messages and the EventHandler will (you guessed it) HANDLE
                    //          them
                    None
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
