pub mod sections_state;
pub mod vim_emulator;

use sections_state::{client_config_state::ClientConfigState, public_keys_list_state::NewPublicKeyFocus};
use vim_emulator::VimMode;

use crate::{
    commands::{self, CmdTask},
    events::messages::Message,
    ui::{
        color_variants::ColorVariant,
        components::{popups::Popup, sections::Section},
        Focus,
    },
};

use self::sections_state::SectionsStates;

#[derive(Default, Clone, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

/// Struct in charge of handling the application's state
///
/// Changes made to this struct MUST be issued using the 'update' method, which
/// updates the state based on the given Message
#[derive(Default)]
pub struct Model {
    running_state: RunningState,
    /// Don't manually modify it. Use self.set_popup instead.
    current_popup: Option<Popup>,
    current_section: Section,
    current_focus: Focus,
    sections_states: SectionsStates,
    /// Indicates which, if any, is the currently running command
    /// NOTE: For now, there'll only be a single command running at a time. In the future, I'm
    /// thinking of being able to allow multiple commands at a time
    current_commands: Vec<commands::CmdTask>,
    /// When this field is Some, the content must the displayed in an error popup and ANY input
    /// must end in the process termination
    fatal_error: Option<String>,
    /// The error message that should be displayed in the error popup
    current_error: Option<String>,
}

impl Model {
    pub fn get_popup(&self) -> Option<Popup> {
        self.current_popup.clone()
    }

    /// Returns which view is currently the app focusing on
    pub fn get_focus(&self) -> Focus {
        self.current_focus.clone()
    }

    pub fn is_app_done(&self) -> bool {
        self.running_state == RunningState::Done
    }

    /// Updates the model state based on the given message
    pub fn update(&mut self, message: Message) {
        match message {
            Message::StopApp => {
                self.running_state = RunningState::Done;
            }
            Message::ShowPopup(popup) => self.set_popup(Some(popup)),
            Message::HidePopup => self.set_popup(None),
            Message::MoveToNextSection => self.next_section(),
            Message::MoveToPrevSection => self.prev_section(),
            Message::SelNextListItem => {
                if let Focus::Section(section) = self.current_focus {
                    match section {
                        Section::KnownHostsList => self
                            .sections_states
                            .get_known_hosts_list_state_mut()
                            .next_item(),
                        Section::PublicKeysList => self
                            .sections_states
                            .get_public_keys_list_state_mut()
                            .next_item(),
                        Section::ClientConfig => {}
                    }
                }
            }
            Message::SelPrevListItem => {
                if let Focus::Section(section) = self.current_focus {
                    match section {
                        Section::KnownHostsList => self
                            .sections_states
                            .get_known_hosts_list_state_mut()
                            .prev_item(),
                        Section::PublicKeysList => self
                            .sections_states
                            .get_public_keys_list_state_mut()
                            .prev_item(),
                        Section::ClientConfig => {}
                    }
                }
            }
            Message::SelNextPopupItem => {
                if let Focus::Popup(ref popup) = self.current_focus {
                    match popup {
                        Popup::AddPubKey => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();
                            new_key_state.next_focus();
                        }
                        Popup::ExitPrompt => {}
                        _ => {}
                    }
                }
            }
            Message::SelPrevPopupItem => {
                if let Focus::Popup(ref popup) = self.current_focus {
                    match popup {
                        Popup::AddPubKey => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();
                            new_key_state.prev_focus();
                        }
                        Popup::ExitPrompt => {}
                        _ => {}
                    }
                }
            }
            Message::WriteChar(ch) => {
                if let Focus::Popup(ref popup) = self.current_focus {
                    match popup {
                        Popup::AddPubKey => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();

                            match new_key_state.get_focus() {
                                NewPublicKeyFocus::Name => new_key_state.write_name(ch),
                                NewPublicKeyFocus::Comment => new_key_state.write_comment(ch),
                            }
                        }
                        Popup::PromptPassphrase => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();

                            new_key_state.write_passphrase(ch);
                        }
                        Popup::PromptReenterPassphrase => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();

                            new_key_state.write_passphrase_check(ch);
                        }
                        Popup::ExitPrompt => {}
                        _ => {}
                    }
                }
            }
            Message::PopChar => {
                if let Focus::Popup(ref popup) = self.current_focus {
                    match popup {
                        Popup::AddPubKey => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();

                            match new_key_state.get_focus() {
                                NewPublicKeyFocus::Name => new_key_state.del_name_char(),
                                NewPublicKeyFocus::Comment => new_key_state.del_comment_char(),
                            }
                        }
                        Popup::PromptPassphrase => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();

                            new_key_state.del_passphare_char();
                        }
                        Popup::PromptReenterPassphrase => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();

                            new_key_state.del_passphrase_check_char();
                        }
                        Popup::ExitPrompt => {}
                        _ => {}
                    }
                }
            }
            Message::PopWord => {
                if let Focus::Popup(ref popup) = self.current_focus {
                    match popup {
                        Popup::AddPubKey => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();

                            match new_key_state.get_focus() {
                                NewPublicKeyFocus::Name => new_key_state.del_name_word(),
                                NewPublicKeyFocus::Comment => new_key_state.del_comment_word(),
                            }
                        }
                        Popup::PromptPassphrase => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();

                            new_key_state.del_passphrase();
                        }
                        Popup::PromptReenterPassphrase => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();

                            new_key_state.del_passphrase_check();
                        }
                        Popup::ExitPrompt => {}
                        _ => {}
                    }
                }
            }
            Message::CmdSpawned(cmd_task) => match cmd_task {
                CmdTask::SshKeygen => {
                    self.current_commands.push(cmd_task);
                    self.current_popup = Some(Popup::WaitingCmd);
                }
            },
            Message::CmdFinished => {
                self.current_commands.clear();
                self.set_popup(None);
            }
            Message::PrintSuccess(success_str) => {
                self.set_popup(Some(Popup::WithCfg(success_str, ColorVariant::Success)));
            }
            Message::PrintError(error_str) => {
                self.current_error = Some(error_str);
                self.set_popup(Some(Popup::ErrorMsg));
            }
            Message::FatalError(error_str) => {
                self.fatal_error = Some(error_str.clone());
                self.set_popup(Some(Popup::WithCfg(
                    error_str.clone(),
                    ColorVariant::Danger,
                )));
            }
            Message::PromptNewKeyPassphrase => self.set_popup(Some(Popup::PromptPassphrase)),
            Message::PromptReenterNewKeyPassPhrase => {
                self.set_popup(Some(Popup::PromptReenterPassphrase))
            }
            Message::CleanNewKeyPassphraseInput => {
                self.sections_states
                    .get_public_keys_list_state_mut()
                    .get_new_key_state_mut()
                    .clean_passphrases();
            }
            Message::RefreshPublicKeysList => {
                self.sections_states
                    .get_public_keys_list_state_mut()
                    .load_public_keys();
            }
            Message::PromptKeyOverwrite => {
                self.set_popup(Some(Popup::PromptKeyOverwrite));
            }
            Message::RefreshKnownHostsList => {
                self.sections_states
                    .get_known_hosts_list_state_mut()
                    .load_known_hosts();
            }
            Message::PromptDeleteKeyPairConfirmation => {
                self.set_popup(Some(Popup::PromptDeleteKeyPairConfirmation));
            }
            Message::TextAreaInteract => {
                let ccstate = self.get_client_config_state_mut();
                ccstate.set_vim_mode(VimMode::Normal);
                ccstate.enter_interactive();
            }
            Message::VimQuit => {
                self.get_client_config_state_mut().quit_interactive();
            }
            Message::SetVimMode(mode) => {
                self.get_client_config_state_mut().set_vim_mode(mode);
            }
            Message::TextAreaMoveCursor(cursor_move) => {
                self.get_client_config_state_mut().move_cursor(cursor_move);
            }
            Message::TextAreaInput(input) => {
                self.get_client_config_state_mut().handle_textarea_input(input);
            }
            Message::TextAreaUndo => {
                self.get_client_config_state_mut().textarea_undo();
            }
            Message::TextAreaRedo => {
                self.get_client_config_state_mut().textarea_redo();
            }
            Message::TextAreaYank => {
                self.get_client_config_state_mut().textarea_yank();
            }
            Message::TextAreaPaste => {
                self.get_client_config_state_mut().textarea_paste();
            }
            Message::TextAreaWriteBuffer => {
                self.get_client_config_state_mut().textarea_write_buffer();
            }
            Message::TextAreaScroll(scroll) => {
                self.get_client_config_state_mut().textarea_scroll(scroll);
            }
            _ => {}
        }
    }

    pub fn get_sections_state(&self) -> &SectionsStates {
        &self.sections_states
    }

    pub fn get_current_command(&self) -> Option<commands::CmdTask> {
        if let Some(cmd_task) = self.current_commands.first() {
            return Some(*cmd_task);
        }

        None
    }

    pub fn get_fatal_error(&self) -> Option<String> {
        self.fatal_error.clone()
    }

    pub fn get_current_error(&self) -> Option<String> {
        self.current_error.clone()
    }

    pub fn get_current_section(&self) -> Section {
        self.current_section
    }

    pub fn get_client_config_state(&self) -> &ClientConfigState {
        self.sections_states.get_client_config_state()
    }

    /// Setting a popup should also set the current section
    fn set_popup(&mut self, new_popup: Option<Popup>) {
        match self.current_popup {
            Some(Popup::ErrorMsg) if new_popup.clone().is_some_and(|v| v != Popup::ErrorMsg) => {
                self.current_error = None
            }
            _ => {}
        }

        if let Some(ref popup) = new_popup {
            self.current_focus = Focus::Popup(popup.clone());
        } else {
            self.current_focus = Focus::Section(self.current_section);
        }
        self.current_popup = new_popup;
    }

    fn next_section(&mut self) {
        let next_section = self.sections_states.next_section();
        self.current_section = next_section;
        self.current_focus = Focus::Section(next_section);
    }

    fn prev_section(&mut self) {
        let prev_section = self.sections_states.prev_section();
        self.current_section = prev_section;
        self.current_focus = Focus::Section(prev_section);
    }

    fn get_client_config_state_mut(&mut self) -> &mut ClientConfigState {
        self.sections_states.get_client_config_state_mut()
    }
}
