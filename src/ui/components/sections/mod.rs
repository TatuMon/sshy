pub mod known_hosts_list;
pub mod public_keys_list;

use serde::Serialize;

/// Sections define where the user currently is.
///
/// These are needed so we can have keybindings do different things depending on
/// the current active section
#[derive(Clone, Serialize, Default, PartialEq, Copy)]
pub enum Section {
    #[default]
    KnownHostsList,
    PublicKeysList
}
