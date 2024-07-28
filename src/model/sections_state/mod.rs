use serde::Serialize;

use crate::ui::components::sections::Section;

use self::{
    known_hosts_list_state::KnownHostsListState, public_keys_list_state::PublicKeysListState,
};

pub mod known_hosts_list_state;
pub mod public_keys_list_state;

#[derive(Clone, Default, Serialize)]
pub struct SectionsStates {
    known_hosts_list_state: KnownHostsListState,
    public_keys_list_state: PublicKeysListState,
}

impl SectionsStates {
    pub fn set_focus(&mut self, section: Section) {
        match section {
            Section::KnownHostsList => self.known_hosts_list_state.focus(),
            Section::PublicKeysList => self.public_keys_list_state.focus(),
        }
    }

    pub fn get_known_hosts_list_state(&self) -> &KnownHostsListState {
        &self.known_hosts_list_state
    }

    pub fn get_public_keys_list_state(&self) -> &PublicKeysListState {
        &self.public_keys_list_state
    }
}
