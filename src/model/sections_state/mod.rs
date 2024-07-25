use serde::Serialize;

use self::{known_hosts_list_state::KnownHostsListState, public_keys_list_state::PublicKeysListState};

pub mod known_hosts_list_state;
pub mod public_keys_list_state;

#[derive(Clone, Default, Serialize)]
pub struct SectionsState {
    known_hosts_list_state: KnownHostsListState,
    public_keys_list_state: PublicKeysListState
}

impl SectionsState {
    pub fn set_focus() {

    }
}
