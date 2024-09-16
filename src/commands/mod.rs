pub mod ssh_keygen;

use std::sync::mpsc;

use color_eyre::eyre::Result;
use serde::Serialize;

use crate::events::messages::Message;

pub type TaskMessageRx = mpsc::Receiver<Message>;
pub type TaskMessageTx = mpsc::Sender<Message>;

/// List of available command tasks
#[derive(Clone, Serialize, Copy)]
pub enum CmdTask {
    SshKeygen
}

pub trait Task {
    fn start(&mut self, task_msg_tx: TaskMessageTx) -> Result<()>;

    /// After this call, the task becomes unusable
    fn terminate(&mut self) -> Result<()>;
}
