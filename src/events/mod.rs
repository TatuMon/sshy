pub mod messages;

use std::{collections::VecDeque, sync::mpsc, time::Duration};

use color_eyre::eyre::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    commands::{self, ssh_keygen::SshKeygenCmd, CmdTask, Task},
    model::Model,
    ui::{
        components::{popups::Popup, sections::Section},
        Focus,
    },
};

use self::messages::Message;

pub struct EventHandler {
    // Used for communication with commands and subprocesses
    task_msg_rx: commands::TaskMessageRx,
    task_msg_tx: commands::TaskMessageTx,
    running_cmd: Option<Box<dyn Task>>
}

impl Default for EventHandler {
    fn default() -> Self {
        let (task_msg_tx, task_msg_rx) = mpsc::channel::<Message>();

        Self {
            task_msg_rx,
            task_msg_tx,
            running_cmd: None
        }
    }
}

impl EventHandler {
    /// Returns an iterator over a list of messages created based on the newly
    /// polled events
    ///
    /// # What events?
    /// These events could be user input, incoming data of the SSH session, etc.
    ///
    /// # In which order are messages added to the queue?
    ///     1. User input events
    ///     2. Commands and subprocesses events, in the order they were received
    pub fn poll_messages(&mut self, model: &Model) -> Result<impl Iterator<Item = Message>> {
        let mut queue = VecDeque::new();

        if let Some(event) = self.poll_crossterm_event()? {
            let crossterm_event_msg = match event {
                Event::Key(key_event) => self.handle_key_event(key_event, model),
                Event::Resize(_, _) => Some(Message::Draw),
                // Event::Mouse(mouse_event) => Ok(handle_mouse_event(mouse_event)),
                _ => None,
            };

            if let Some(msg) = crossterm_event_msg {
                queue.push_back(msg);
            }
        };

        Ok(queue.into_iter())
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
    /// Matching the code first, makes it impossible to write to an input
    fn handle_key_event(&mut self, key: KeyEvent, model: &Model) -> Option<Message> {
        match model.get_focus() {
            Focus::Section(section) => self.handle_section_key_event(section, key),
            Focus::Popup(popup) => self.handle_popup_key_event(popup, key, model),
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

    /// Starts the given command task
    ///
    /// # Returns
    /// Returns either:
    ///     - Message::CmdSpawned(cmd_task)
    ///     - Message::PrintError(error_str)
    fn start_command(&mut self, cmd_task: commands::CmdTask, model: &Model) -> Message {
        let msg_tx_cp = self.task_msg_tx.clone();

        match cmd_task {
            CmdTask::SshKeygen => {
                let mut cmd = SshKeygenCmd::new(
                    model
                        .get_sections_state()
                        .get_public_keys_list_state()
                        .get_new_key_state(),
                );
                match cmd.start(msg_tx_cp) {
                    Err(err) => Message::PrintError(err.to_string()),
                    _ => {
                        self.running_cmd = Some(Box::new(cmd));
                        Message::CmdSpawned(cmd_task)
                    },
                }
            }
        }
    }

    /// Terminates the currently running command
    ///
    /// # Returns
    /// Returns either:
    ///     - Message::CmdFinished
    ///     - Message::PrintError
    fn terminate_command(&mut self) -> Message {
        match &mut self.running_cmd {
            None => Message::PrintError(String::from("no command is running")),
            Some(cmd) => {
                let termination = cmd.terminate().wrap_err("failed to terminate command");
                match termination {
                    Err(err) => Message::PrintError(err.to_string()),
                    _ => Message::CmdFinished
                }
            }
        }
    }

    fn handle_popup_key_event(
        &mut self,
        popup: Popup,
        key: KeyEvent,
        model: &Model,
    ) -> Option<Message> {
        match key.code {
            KeyCode::Char(ch) => match popup {
                Popup::ExitPrompt => {
                    if ch == 'q' {
                        let _ = self.terminate_command();
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
                    Some(self.start_command(CmdTask::SshKeygen, model))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
