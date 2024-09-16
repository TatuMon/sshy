use core::panic;
use std::process::{self, Command, Stdio};

use color_eyre::eyre::{eyre, Context, Result};

use crate::{
    events::messages::Message, model::sections_state::public_keys_list_state::NewPublicKeyState,
};

use super::Task;

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
    task_msg_tx: Option<super::TaskMessageTx>,
    cmd_handle: Option<std::process::Child>
}

impl Task for SshKeygenCmd {
    fn start(&mut self, task_msg_tx: super::TaskMessageTx) -> Result<()> {
        let args: [&str;6] = [
            "-t", self.keytype.into(),
            "-f", &self.filename,
            "-C", &self.comment
        ];        

        let cmd_handle = Command::new("ssh-keygen")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().wrap_err("error spawning ssh-keygen")?;

        self.cmd_handle = Some(cmd_handle);
        self.task_msg_tx = Some(task_msg_tx);
        self.running = true;

        Ok(())
    }

    /// Terminates the command
    ///
    /// # Panics
    /// As this function must not be called before starting the command, it
    /// panics if self.cmd_handle is None
    fn terminate(&mut self) -> Result<()> {
        match &mut self.cmd_handle {
            None => panic!("no running command"),
            Some(handle) => handle.kill().wrap_err("failed to terminate command")
        }
    }
}

impl SshKeygenCmd {
    pub fn new(state: &NewPublicKeyState) -> Self {
        Self {
            keytype: state.get_type(),
            filename: state.get_name().into(),
            comment: state.get_comment().into(),
            passphrase: None,
            running: false,
            task_msg_tx: None,
            cmd_handle: None
        }
    }
}
