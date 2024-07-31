pub mod exit_prompt;
pub mod debug_model;
pub mod add_pub_key;

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub enum Popup {
    ExitPrompt,
    AddPubKey,
    /// Popup used to debug the model's state
    DebugModel
}
