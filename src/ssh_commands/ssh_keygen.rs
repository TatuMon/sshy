use std::sync::mpsc::Sender;

use crate::{
    events::messages::Message, model::sections_state::public_keys_list_state::NewPublicKeyState,
};

#[derive(Clone, Copy, Default)]
pub enum PublicKeyType {
    #[default]
    ED25519,
}

impl From<PublicKeyType> for &str {
    fn from(value: PublicKeyType) -> Self {
        match value {
            PublicKeyType::ED25519 => "ed25519",
        }
    }
}

pub struct SshKeygenCmd {
    keytype: PublicKeyType,
    filename: String,
    passphrase: Option<String>,
    comment: String,
    running: bool,
    msg_tx: Sender<Message>,
}

impl SshKeygenCmd {
    fn new(state: &NewPublicKeyState, msg_tx: Sender<Message>) -> Self {
        Self {
            keytype: state.get_type(),
            filename: state.get_name().into(),
            comment: state.get_comment().into(),
            passphrase: None,
            running: false,
            msg_tx,
        }
    }
}
