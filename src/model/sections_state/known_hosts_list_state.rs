use ratatui::widgets::ListState;
use serde::Serialize;

use crate::utils;

#[derive(Clone, Default)]
pub struct KnownHostsListState {
    items: Vec<String>,
    list_state: ListState,
    has_focus: bool
}

impl KnownHostsListState {
    pub fn load_known_hosts(&mut self) {
        let known_hosts = utils::files::get_known_hosts().unwrap_or_default();
        self.items = known_hosts;
    }

    pub fn focus(&mut self) {
        self.has_focus = true;
    }

    pub fn unfocus(&mut self) {
        self.has_focus = false;
    }

    pub fn has_focus(&self) -> bool {
        self.has_focus
    }
}

impl Serialize for KnownHostsListState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("serializer not implemented")
    }
}
