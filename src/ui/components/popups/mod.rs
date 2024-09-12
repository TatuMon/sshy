pub mod add_pub_key;
pub mod debug_model;
pub mod exit_prompt;

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub enum Popup {
    ExitPrompt,
    AddPubKey,
    /// Popup used to debug the model's state
    DebugModel,
}
