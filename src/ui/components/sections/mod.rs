pub mod known_hosts_list;
pub mod public_keys_list;

use std::fmt::Display;

use serde::Serialize;

/// Sections define where the user currently is.
///
/// These are needed so we can have keybindings do different things depending on
/// the current active section
#[derive(Clone, Serialize, Default, PartialEq, Copy)]
pub enum Section {
    #[default]
    KnownHostsList,
    PublicKeysList,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Section::KnownHostsList => write!(f, "Known Hosts List"),
            Section::PublicKeysList => write!(f, "Public Keys List")
        }
    }
}
