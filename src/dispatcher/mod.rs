use color_eyre::eyre::Result;

use crate::{actions::Action, stores::{app_state_store::AppStateStore, known_hosts_store::KnownHostsStore}};

pub struct Dispatcher {
    app_state_store: &mut AppStateStore,
    known_hosts_store: &mut KnownHostsStore
}

impl Dispatcher {
    pub fn default() -> Self {
        Self {
            app_state_store: AppStateStore::default(),
            known_hosts_store: KnownHostsStore::default()
        }
    }

    pub fn dispatch(&mut self, action: Action) -> Result<()> {
        match action {
            Action::AppStateAction(app_state_action) => {
                self.app_state_store.update(app_state_action)
            },
            Action::KnownHostsAction(known_hosts_action) => {
                self.known_hosts_store.update(known_hosts_action)
            }
        }
    }
}
