pub mod messages;

use std::{
    collections::{HashMap, VecDeque},
    sync::mpsc::{self, TryRecvError},
    time::Duration,
};

use color_eyre::eyre::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    async_jobs::{self, AsyncJob},
    commands::{self, ssh_keygen::SshKeygenCmd, CmdTask},
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
            let crossterm_event_msgs = match event {
                Event::Key(key_event) => self.handle_key_event(key_event, model),
                Event::Resize(_, _) => vec![Message::Draw],
                // Event::Mouse(mouse_event) => Ok(handle_mouse_event(mouse_event)),
                _ => vec![],
            };

            for msg in crossterm_event_msgs {
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
    fn handle_key_event(&mut self, key: KeyEvent, model: &Model) -> Vec<Message> {
        if let Some(_) = model.get_fatal_error() {
            return vec![Message::StopApp];
        }

        match model.get_focus() {
            Focus::Section(current_section) => self.handle_section_key_event(current_section, key),
            Focus::Popup(current_popup) => self.handle_popup_key_event(current_popup, key, model),
        }
    }

    fn handle_section_key_event(&self, current_section: Section, event: KeyEvent) -> Vec<Message> {
        match event.code {
            KeyCode::Char('q') => vec![Message::ShowPopup(Popup::ExitPrompt)],
            KeyCode::Char('p') => vec![Message::ShowPopup(Popup::DebugModel)],
            KeyCode::Right => vec![Message::MoveToNextSection],
            KeyCode::Left => vec![Message::MoveToPrevSection],
            KeyCode::Up => vec![Message::SelPrevListItem],
            KeyCode::Down => vec![Message::SelNextListItem],
            KeyCode::Char('n') => {
                if let Section::PublicKeysList = current_section {
                    vec![Message::ShowPopup(Popup::AddPubKey)]
                } else {
                    vec![]
                }
            }
            KeyCode::Char('R') => {
                match current_section {
                    Section::PublicKeysList => vec!(Message::RefreshPublicKeysList),
                    Section::KnownHostsList => vec!(Message::RefreshKnownHostsList)
                }
            }
            _ => vec!(),
            KeyCode::Char('d') => {
                if let Section::PublicKeysList = current_section {
                    vec![Message::PromptDeleteKeyPairConfirmation]
                } else {
                    vec![]
                }
            }
            _ => vec![],
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

        let name_validation = model.get_sections_state().get_public_keys_list_state().get_new_key_state().validate_name();
        if let Err(validation_err) = name_validation {
            return Message::PrintError(validation_err);
        }

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
                Ok(_) => None,
            };
        }

        None
    }

    fn run_async_job(&mut self, async_job: AsyncJob, model: &Model) {
        let msg_tx = self.task_msg_tx.clone();

        match async_job {
            AsyncJob::DeleteKayPair => {
                let key_name = model
                    .get_sections_state()
                    .get_public_keys_list_state()
                    .get_selected_key_name()
                    .expect("must select a key to delete");
                async_jobs::delete_key_pair::delete_key_pair(key_name, msg_tx);
            }
        }
    }

    fn handle_popup_key_event(
        &mut self,
        current_popup: Popup,
        key: KeyEvent,
        model: &Model,
    ) -> Vec<Message> {
        match key.code {
            KeyCode::Char(ch) => match current_popup {
                Popup::ExitPrompt => {
                    if ch == 'q' {
                        vec![Message::StopApp]
                    } else {
                        vec![]
                    }
                }
                Popup::AddPubKey | Popup::PromptPassphrase
                    if key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    if ch == 'w' {
                        vec![Message::PopWord]
                    } else {
                        vec![]
                    }
                }
                Popup::AddPubKey | Popup::PromptPassphrase | Popup::PromptReenterPassphrase => {
                    vec![Message::WriteChar(ch)]
                }
                Popup::WaitingCmd => match model.get_current_command() {
                    None => vec![Message::HidePopup],
                    Some(cmd_task) => vec![self.kill_command(cmd_task)],
                },
                _ => vec![],
            },
            KeyCode::Backspace => vec![Message::PopChar],
            KeyCode::Esc => match model.get_current_command() {
                None => vec![Message::HidePopup],
                Some(cmd_task) => vec![self.kill_command(cmd_task)],
            },
            KeyCode::Tab => vec![Message::SelNextPopupItem],
            KeyCode::BackTab => vec![Message::SelPrevPopupItem],
            KeyCode::Enter => match current_popup {
                Popup::AddPubKey => vec![self.start_command(CmdTask::SshKeygen, model)],
                Popup::PromptPassphrase => {
                    if let Some(msg) = self.write_to_cmd(
                        CmdTask::SshKeygen,
                        &model
                            .get_sections_state()
                            .get_public_keys_list_state()
                            .get_new_key_state()
                            .get_passphrase_bytes(),
                    ) {
                        vec![msg]
                    } else {
                        vec![]
                    }
                }
                Popup::PromptReenterPassphrase => {
                    let mut msgs: Vec<Message> = vec![];

                    if let Some(msg) = self.write_to_cmd(
                        CmdTask::SshKeygen,
                        &model
                            .get_sections_state()
                            .get_public_keys_list_state()
                            .get_new_key_state()
                            .get_passphrase_check_bytes(),
                    ) {
                        msgs.push(msg);
                    };
                    msgs.push(Message::CleanNewKeyPassphraseInput);

                    msgs
                }
                Popup::PromptKeyOverwrite => {
                    let mut msgs: Vec<Message> = vec![];

                    if let Some(msg) = self.write_to_cmd(CmdTask::SshKeygen, "y".as_bytes()) {
                        msgs.push(msg);
                    };

                    msgs
                }
                Popup::PromptDeleteKeyPairConfirmation => {
                    self.run_async_job(AsyncJob::DeleteKayPair, model);
                    vec!()
                }
                _ => vec!(),
            },
            _ => vec!(),
        }
    }
}
