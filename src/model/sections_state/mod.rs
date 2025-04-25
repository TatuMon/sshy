use serde::Serialize;

use crate::ui::components::sections::Section;

use self::{
    known_hosts_list_state::KnownHostsListState,
    public_keys_list_state::PublicKeysListState,
    client_config_state::ClientConfigState
};

pub mod known_hosts_list_state;
pub mod public_keys_list_state;
pub mod client_config_state;

const SECTIONS_ORDER: [Section; 3] = [Section::KnownHostsList, Section::PublicKeysList, Section::ClientConfig];

#[derive(Default, Serialize)]
pub struct SectionsStates {
    known_hosts_list_state: KnownHostsListState,
    public_keys_list_state: PublicKeysListState,
    client_config_state: ClientConfigState
}

impl SectionsStates {
    /// Moves the focus to the next section in the order
    pub fn next_section(&mut self) -> Section {
        for (idx, section) in SECTIONS_ORDER.iter().enumerate() {
            if self.has_focus_on(section.to_owned()) {
                // Using unwrap and direct array access because SECTIONS_ORDER is constant
                if section == SECTIONS_ORDER.last().unwrap() {
                    return section.to_owned();
                } else {
                    let next_section = SECTIONS_ORDER[idx + 1];
                    self.set_focus(next_section);
                    return next_section;
                }
            }
        }

        Section::KnownHostsList
    }

    /// Moves the focus to the previous section in the order
    /// Or the first one if already in last one
    pub fn prev_section(&mut self) -> Section {
        for (idx, section) in SECTIONS_ORDER.iter().enumerate() {
            if self.has_focus_on(section.to_owned()) {
                // Using unwrap and direct array access because SECTIONS_ORDER is constant
                if section == SECTIONS_ORDER.first().unwrap() {
                    return section.to_owned();
                } else {
                    let prev_section = SECTIONS_ORDER[idx - 1];
                    self.set_focus(prev_section);
                    return prev_section;
                }
            }
        }

        Section::KnownHostsList
    }

    pub fn set_focus(&mut self, section: Section) {
        match section {
            Section::KnownHostsList => {
                self.public_keys_list_state.unfocus();
                self.client_config_state.unfocus();
                self.known_hosts_list_state.focus();
            }
            Section::PublicKeysList => {
                self.known_hosts_list_state.unfocus();
                self.client_config_state.unfocus();
                self.public_keys_list_state.focus();
            }
            Section::ClientConfig => {
                self.known_hosts_list_state.unfocus();
                self.public_keys_list_state.unfocus();
                self.client_config_state.focus();
            }
        }
    }

    pub fn has_focus_on(&self, section: Section) -> bool {
        match section {
            Section::KnownHostsList => self.known_hosts_list_state.has_focus(),
            Section::PublicKeysList => self.public_keys_list_state.has_focus(),
            Section::ClientConfig => self.client_config_state.has_focus()
        }
    }

    pub fn get_known_hosts_list_state(&self) -> &KnownHostsListState {
        &self.known_hosts_list_state
    }

    pub fn get_public_keys_list_state(&self) -> &PublicKeysListState {
        &self.public_keys_list_state
    }

    pub fn get_client_config_state(&self) -> &ClientConfigState {
        &self.client_config_state
    }

    pub fn get_known_hosts_list_state_mut(&mut self) -> &mut KnownHostsListState {
        &mut self.known_hosts_list_state
    }

    pub fn get_public_keys_list_state_mut(&mut self) -> &mut PublicKeysListState {
        &mut self.public_keys_list_state
    }

    pub fn get_client_config_state_mut(&mut self) -> &mut ClientConfigState {
        &mut self.client_config_state
    }
}
