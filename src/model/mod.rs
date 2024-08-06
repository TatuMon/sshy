pub mod sections_state;

use sections_state::public_keys_list_state::NewPublicKeyFocus;
use serde::Serialize;

use crate::{
    events::messages::Message,
    ui::{
        components::{popups::Popup, sections::Section},
        Focus,
    },
};

use self::sections_state::SectionsStates;

#[derive(Default, Clone, PartialEq, Serialize)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

/// Struct in charge of handling the application's state
///
/// Changes made to this struct MUST be issued using the 'update' method, which
/// updates the state based on the given Message
#[derive(Default, Serialize)]
pub struct Model {
    running_state: RunningState,
    current_popup: Option<Popup>,
    current_focus: Focus,
    previous_focus: Focus,
    sections_states: SectionsStates,
}

impl Model {
    pub fn get_popup(&self) -> Option<Popup> {
        self.current_popup
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
                    }
                }
            }
            Message::SelNextPopupItem => {
                if let Focus::Popup(popup) = self.current_focus {
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
                if let Focus::Popup(popup) = self.current_focus {
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
                if let Focus::Popup(popup) = self.current_focus {
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
                        Popup::ExitPrompt => {}
                        _ => {}
                    }
                }
            }
            Message::PopChar => {
                if let Focus::Popup(popup) = self.current_focus {
                    match popup {
                        Popup::AddPubKey => {
                            let new_key_state = self
                                .sections_states
                                .get_public_keys_list_state_mut()
                                .get_new_key_state_mut();

                            match new_key_state.get_focus() {
                                NewPublicKeyFocus::Name => new_key_state.del_name(),
                                NewPublicKeyFocus::Comment => new_key_state.del_comment(),
                            }
                        }
                        Popup::ExitPrompt => {}
                        _ => {}
                    }
                }
            }
            Message::PopWord => {
                if let Focus::Popup(popup) = self.current_focus {
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
                        Popup::ExitPrompt => {}
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    /// Returns if there's a popup currently being shown
    pub fn on_popup(&self) -> bool {
        self.current_popup.is_some()
    }

    pub fn get_sections_state(&self) -> &SectionsStates {
        &self.sections_states
    }

    /// Setting a popup should also set the current section
    fn set_popup(&mut self, popup: Option<Popup>) {
        self.current_popup = popup;
        if let Some(popup) = popup {
            self.previous_focus = self.current_focus.clone();
            self.current_focus = Focus::Popup(popup);
        } else {
            self.current_focus = self.previous_focus.clone();
        }
    }

    fn next_section(&mut self) {
        let next_section = self.sections_states.next_section();
        self.current_focus = Focus::Section(next_section);
    }

    fn prev_section(&mut self) {
        let prev_section = self.sections_states.prev_section();
        self.current_focus = Focus::Section(prev_section);
    }
}
