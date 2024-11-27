pub mod add_pub_key;
pub mod debug_model;
pub mod exit_prompt;
pub mod waiting_cmd;
pub mod error_msg;
pub mod set_pub_key_passphrase;

use serde::Serialize;

#[derive(Clone, Copy, Serialize, PartialEq, Eq)]
pub enum Popup {
    ExitPrompt,
    AddPubKey,
    /// Popup used to debug the model's state
    DebugModel,
    WaitingCmd,
    ErrorMsg,
    PromptPassphrase
}
