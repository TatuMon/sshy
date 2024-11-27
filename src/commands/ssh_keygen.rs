use std::io;

use color_eyre::eyre::{eyre, Result};

use crate::events::messages::Message;
use crate::model::sections_state::public_keys_list_state::NewPublicKeyState;

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
    comment: String,
}

impl Task for SshKeygenCmd {
    /// Starts the ssh-keygen command, creating a detached green-thread in charge
    /// of handling the messaging between the the command and the app.
    fn start(
        new_key: &NewPublicKeyState,
        task_msg_tx: super::TaskMessageTx,
    ) -> Result<super::CmdWriterEnd> {
        let cmd = SshKeygenCmd {
            keytype: new_key.get_type(),
            filename: new_key.get_name().into(),
            comment: new_key.get_comment().into(),
        };

        let args: [&str; 6] = [
            "-t",
            cmd.keytype.into(),
            "-f",
            &cmd.filename,
            "-C",
            &cmd.comment,
        ];

        let pty_system = portable_pty::native_pty_system();
        let pty_pair = pty_system
            .openpty(portable_pty::PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| eyre!("{}", e))?;

        let mut cmd_builder = portable_pty::CommandBuilder::new("ssh-keygen");
        cmd_builder.args(args);

        let child_cmd = pty_pair
            .slave
            .spawn_command(cmd_builder)
            .map_err(|e| eyre!("error spawning ssh-keygen: {}", e))?;

        let pty_reader = pty_pair
            .master
            .try_clone_reader()
            .map_err(|e| eyre!("error getting command reader: {}", e))?;

        tokio::spawn(handle_ssh_keygen(super::CmdReaderEnd {
            reader: pty_reader,
            msg_sender: task_msg_tx,
        }));

        let writer = pty_pair
            .master
            .take_writer()
            .map_err(|e| eyre!("error getting command writer: {}", e))?;

        let child_killer = child_cmd.clone_killer();

        let writer_end = super::CmdWriterEnd {
            writer,
            child_killer,
        };

        Ok(writer_end)
    }
}

fn handle_ssh_keygen_output(content: &[u8]) -> Result<Message> {
    const SET_PASSPHRASE_PROMPT: &[u8] = b"Enter passphrase (empty for no passphrase)";
    if content
        .windows(SET_PASSPHRASE_PROMPT.len())
        .any(|b| b == SET_PASSPHRASE_PROMPT)
    {
        return Ok(Message::PromptNewKeyPassphrase);
    }

    return Ok(Message::Draw);
}

async fn handle_ssh_keygen(mut reader_end: super::CmdReaderEnd) {
    let mut buf = [0u8; 1024];
    loop {
        match reader_end.reader.read(&mut buf) {
            Ok(0) => {
                // EOF reached
                let _ = reader_end
                    .msg_sender
                    .send(Message::CmdFinished)
                    .expect("failed to terminate child command");
                break;
            }
            Ok(_n) => {
                let msg = handle_ssh_keygen_output(&buf)
                    .map_err(|e| eyre!("error handling command output: {}", e));
                match msg {
                    Err(e) => reader_end
                        .msg_sender
                        .send(Message::PrintError(e.to_string()))
                        .unwrap(),
                    Ok(msg) => reader_end.msg_sender.send(msg).unwrap(),
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    // No data available; optionally sleep or perform other work
                    continue;
                } else {
                    panic!("error reading from PTY: {}", e);
                }
            }
        };
    }
}
