pub mod exit_prompt;
pub mod debug_model;

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub enum Popup {
    ExitPrompt,
    /// Popup used to debug the model's state
    DebugModel
}
