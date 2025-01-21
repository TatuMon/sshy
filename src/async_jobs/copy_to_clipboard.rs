use std::sync::mpsc;

use copypasta::{ClipboardContext, ClipboardProvider};

use crate::events::messages::Message;

/// Asynchronously deletes a file, creating a green thread that communicates via the msg_tx
pub fn copy_to_clipboard(content: String, msg_tx: mpsc::Sender<Message>) {
    tokio::spawn(handle_copy_to_clipboard(content, msg_tx));
}

async fn handle_copy_to_clipboard(content: String, msg_tx: mpsc::Sender<Message>) {
    match ClipboardContext::new() {
        Ok(mut ctx) => match ctx.set_contents(content) {
            Ok(_) => {
                msg_tx.send(Message::PrintSuccess(String::from(
                    "Public key copied to clipboard",
                ))).expect("Failed to send success message");
            }
            Err(err) => {
                msg_tx.send(Message::PrintError(format!(
                    "Failed to copy to clipboard: {}",
                    err.to_string()
                ))).expect("Failed to send error notice");
            }
        },
        Err(err) => {
            msg_tx.send(Message::PrintError(format!(
                "Failed to load clipboard: {}",
                err.to_string()
            ))).expect("Failed to send error notice");
        }
    };
}
