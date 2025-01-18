pub mod ssh_keygen;

use crate::model::sections_state::public_keys_list_state::NewPublicKeyState;
use portable_pty::ChildKiller;
use std::sync::mpsc;

use color_eyre::eyre::{eyre, Result};

use crate::events::messages::Message;
use crate::utils::constants;

use std::collections::HashMap;

pub type TaskMessageRx = mpsc::Receiver<Message>;
pub type TaskMessageTx = mpsc::Sender<Message>;

/// List of available command tasks
#[derive(Clone, Copy, Eq, Hash, PartialEq, serde::Serialize)]
pub enum CmdTask {
    SshKeygen,
}

pub type PtyReader = Box<dyn std::io::Read + Send>;
pub type PtyWriter = Box<dyn std::io::Write + Send>;

/// A CmdWriterEnd has a writer to write content to the child command and a child killer to kill the
/// child command
///
/// This struct should be handled by the main thread, which forwards the input from the user to the
/// child command
pub struct CmdWriterEnd {
    writer: PtyWriter,
    child_killer: Box<dyn ChildKiller + Send + Sync>,
}

impl CmdWriterEnd {
    pub fn kill_child(&mut self) -> Result<()> {
        self.child_killer
            .kill()
            .map_err(|e| eyre!("failed to kill child command: {}", e))
    }

    pub fn write(&mut self, content: &[u8]) -> Result<()> {
        let content_with_newline: Vec<u8> = content
            .iter()
            .copied()
            .chain(constants::LINE_TERMINATOR.to_owned())
            .collect();

        self.writer
            .write_all(&content_with_newline)
            .map_err(|e| eyre!("failed to write to command: {}", e))?;
        self.writer
            .flush()
            .map_err(|e| eyre!("failed to send data to command: {}", e))
    }
}

/// A CmdReaderEnd has a reader to read content from the child command and a message sender to send
/// Messages to the main thread
///
/// This struct should be handled by the child command' thread, which, based on the read content,
/// will generate the appropiate messages for the main thread to handle
pub struct CmdReaderEnd {
    reader: PtyReader,
    msg_sender: TaskMessageTx,
}

pub type CmdWriterEnds = HashMap<CmdTask, CmdWriterEnd>;

pub trait Task {
    fn start(new_key_input: &NewPublicKeyState, task_msg_tx: TaskMessageTx)
        -> Result<CmdWriterEnd>;
}
