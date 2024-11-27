pub mod messages;

use std::{
    collections::{HashMap, VecDeque},
    sync::mpsc::{self, TryRecvError},
    time::Duration,
};

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
    cmd_writer_ends: commands::CmdWriterEnds,
}

impl Default for EventHandler {
    fn default() -> Self {
        let (task_msg_tx, task_msg_rx) = mpsc::channel::<Message>();

        Self {
            task_msg_rx,
            task_msg_tx,
            cmd_writer_ends: HashMap::new(),
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

        match self.task_msg_rx.try_recv() {
            Err(err) if err != TryRecvError::Empty => {
                queue.push_back(Message::PrintError(err.to_string()))
            }
            Ok(msg) => queue.push_back(msg),
            _ => {}
        }

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
        if let Some(_) = model.get_fatal_error() {
            return Some(Message::StopApp);
        }

        match model.get_focus() {
            Focus::Section(current_section) => self.handle_section_key_event(current_section, key),
            Focus::Popup(current_popup) => self.handle_popup_key_event(current_popup, key, model),
        }
    }

    fn handle_section_key_event(&self, current_section: Section, event: KeyEvent) -> Option<Message> {
        match event.code {
            KeyCode::Char('q') => Some(Message::ShowPopup(Popup::ExitPrompt)),
            KeyCode::Char('p') => Some(Message::ShowPopup(Popup::DebugModel)),
            KeyCode::Right => Some(Message::MoveToNextSection),
            KeyCode::Left => Some(Message::MoveToPrevSection),
            KeyCode::Up => Some(Message::SelPrevListItem),
            KeyCode::Down => Some(Message::SelNextListItem),
            KeyCode::Char('n') => {
                if let Section::PublicKeysList = current_section {
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
                let cmd_startup = SshKeygenCmd::start(
                    model
                        .get_sections_state()
                        .get_public_keys_list_state()
                        .get_new_key_state(),
                    msg_tx_cp,
                );
                match cmd_startup {
                    Err(err) => Message::PrintError(err.to_string()),
                    Ok(cmd_writer_end) => {
                        self.cmd_writer_ends.insert(cmd_task, cmd_writer_end);
                        Message::CmdSpawned(cmd_task)
                    }
                }
            }
        }
    }

    fn kill_command(&mut self, cmd_task: commands::CmdTask) -> Message {
        match self.cmd_writer_ends.get_mut(&cmd_task) {
            Some(writer_end) => match writer_end.kill_child() {
                Err(err) => Message::FatalError(err.to_string()),
                Ok(_) => Message::CmdFinished,
            },
            None => Message::FatalError("ssh-keygen is not currently running".to_string()),
        }
    }

    fn write_to_cmd(&mut self, cmd_task: commands::CmdTask, content: &[u8]) -> Option<Message> {
        if let Some(writer_end) = self.cmd_writer_ends.get_mut(&cmd_task) {
            return match writer_end.write(content) {
                Err(e) => Some(Message::PrintError(e.to_string())),
                Ok(_) => Some(Message::Draw)
            };
        }

        None
    }

    fn handle_popup_key_event(
        &mut self,
        current_popup: Popup,
        key: KeyEvent,
        model: &Model,
    ) -> Option<Message> {
        match key.code {
            KeyCode::Char(ch) => match current_popup {
                Popup::ExitPrompt => {
                    if ch == 'q' {
                        Some(Message::StopApp)
                    } else {
                        None
                    }
                }
                Popup::AddPubKey | Popup::PromptPassphrase if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    if ch == 'w' {
                        Some(Message::PopWord)
                    } else {
                        None
                    }
                }
                Popup::AddPubKey | Popup::PromptPassphrase => Some(Message::WriteChar(ch)),
                Popup::WaitingCmd => match model.get_current_command() {
                    None => Some(Message::HidePopup),
                    Some(cmd_task) => Some(self.kill_command(cmd_task)),
                },
                _ => None,
            },
            KeyCode::Backspace => Some(Message::PopChar),
            KeyCode::Esc => match model.get_current_command() {
                None => Some(Message::HidePopup),
                Some(cmd_task) => Some(self.kill_command(cmd_task)),
            },
            KeyCode::Tab => Some(Message::SelNextPopupItem),
            KeyCode::BackTab => Some(Message::SelPrevPopupItem),
            KeyCode::Enter => {
                match current_popup {
                    Popup::AddPubKey => Some(self.start_command(CmdTask::SshKeygen, model)),
                    Popup::PromptPassphrase => {
                        self.write_to_cmd(
                            CmdTask::SshKeygen,
                            &model.get_sections_state().get_public_keys_list_state().get_new_key_state().get_passphrase_bytes()
                        )
                    }
                    _ => None
                }
            }
            _ => None,
        }
    }
}
