use color_eyre::eyre::Result;

use crate::actions::KnownHostsAction;

#[derive(Default)]
pub struct KnownHostsStore {
    known_hosts: Vec<String>
}

impl KnownHostsStore {
    pub fn update(&mut self, action: KnownHostsAction) -> Result<()> {
        match action {
            KnownHostsAction::LoadKnownHosts => {
                // TO DO
                // Read known_hosts file
                todo!()
            }
        }
    }
}
