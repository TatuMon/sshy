use std::{io, path::PathBuf};

use color_eyre::eyre::{eyre, Result, WrapErr};

use crate::events::messages::Message;
use crate::model::sections_state::public_keys_list_state::NewPublicKeyState;
use crate::ui::{color_variants::ColorVariant, components::{popups::Popup::WithCfg, sections::Section}};

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

        let home_dir =
            PathBuf::from(std::env::var("HOME").wrap_err("couldn't find home directory")?)
                .join(".ssh/")
                .join(&cmd.filename);

        let home_str = home_dir.as_os_str().to_str().ok_or_else(|| eyre!("invalid home directory"))?;

        let args: [&str; 6] = [
            "-t",
            cmd.keytype.into(),
            "-f",
            home_str,
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

fn handle_ssh_keygen_output(content: &[u8]) -> Result<Vec<Message>> {
    let content_string =
        String::from_utf8(content.to_vec()).wrap_err("failed to read output content")?;

    const SET_PASSPHRASE_PROMPT: &str = "Enter passphrase (empty for no passphrase)";
    const SET_REENTER_PASS_PROMPT: &str = "Enter same passphrase again";
    const SUCCESSFUL_KEYGEN: &str = "Your identification has been saved in";
    const EXISTING_KEY: &str = " already exists";
    const NO_SUCH_DIR: &str = " No such file or directory";

    let match_pass_prompt = content_string.contains(SET_PASSPHRASE_PROMPT);
    if match_pass_prompt {
        return Ok(vec!(Message::PromptNewKeyPassphrase));
    }

    let match_reenter_pass_prompt = content_string.contains(SET_REENTER_PASS_PROMPT);
    if match_reenter_pass_prompt {
        return Ok(vec!(Message::PromptReenterNewKeyPassPhrase));
    }

    let match_successful_keygen = content_string.contains(SUCCESSFUL_KEYGEN);
    if match_successful_keygen {
        let succ_popup_msg = Message::ShowPopup(WithCfg(
            content_string,
            ColorVariant::Success,
        ));
        let reload_keys_msg = Message::ReloadPublicKeysList;
        return Ok(vec!(succ_popup_msg, reload_keys_msg));
    }

    let match_key_exists = content_string.contains(EXISTING_KEY);
    if match_key_exists {
        return Ok(vec!(Message::PromptKeyOverwrite));
    }

    let match_no_such_dir = content_string.contains(NO_SUCH_DIR);
    if match_no_such_dir {
        return Ok(vec!(Message::ShowPopup(WithCfg(
            content_string,
            ColorVariant::Danger,
        ))));
    }

    // return Ok(Message::ShowPopup(WithCfg(
    //     content_string,
    //     ColorVariant::Warning,
    // )));
    return Ok(vec!(Message::Draw))
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
                let msgs = handle_ssh_keygen_output(&buf)
                    .map_err(|e| eyre!("error handling command output: {}", e));
                match msgs {
                    Err(e) => reader_end
                        .msg_sender
                        .send(Message::PrintError(e.to_string()))
                        .unwrap(),
                    Ok(msgs) => msgs.into_iter().for_each(|msg| reader_end.msg_sender.send(msg).unwrap()),
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
