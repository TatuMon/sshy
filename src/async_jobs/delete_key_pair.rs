use crate::{
    events::messages::Message,
    ui::{color_variants::ColorVariant, components::popups::Popup},
    utils::files,
};
use std::sync::mpsc;

/// Asynchronously deletes a file, creating a green thread that communicates via the msg_tx
pub fn delete_key_pair(private_key_name: String, msg_tx: mpsc::Sender<Message>) {
    tokio::spawn(handle_delete_key_pair(private_key_name, msg_tx));
}

async fn handle_delete_key_pair(private_key_name: String, msg_tx: mpsc::Sender<Message>) {
    match files::delete_key_pair(&private_key_name) {
        Err(err) => msg_tx
            .send(Message::PrintError(err.to_string()))
            .expect("failed to send error message when deleting key pair"),
        Ok(_) => {
            let text = format!("key '{}' successfully deleted", private_key_name);
            let color = ColorVariant::Success;
            msg_tx
                .send(Message::ShowPopup(Popup::WithCfg(text, color)))
                .expect("failed to send success message after deleting key pair")
        }
    };
    msg_tx
        .send(Message::RefreshPublicKeysList)
        .expect("failed to send keys list refresh message after deleting key pair")
}
