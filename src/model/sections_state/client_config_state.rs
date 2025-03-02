use serde::Serialize;

use crate::utils::{self, types::textarea_state::TextAreaState};

#[derive(Clone)]
pub struct ClientConfigState {
    textarea_state: TextAreaState,
    has_focus: bool,
    /// Indicates if the user entered interactive mode
    interactive_on: bool,
}

impl ClientConfigState {
    pub fn load_content(&mut self) {
        let config_content = utils::files::get_client_config_content().unwrap_or(String::from("FAILED TO LOAD"));
        self.textarea_state.set_content(config_content);
    }

    pub fn get_textarea_state(&self) -> &TextAreaState {
        &self.textarea_state
    }

    pub fn focus(&mut self) {
        self.has_focus = true;
    }

    pub fn unfocus(&mut self) {
        self.has_focus = false;
    }

    pub fn has_focus(&self) -> bool {
        self.has_focus
    }

    /// Indicates if the user is on interactive mode
    pub fn is_interactive_on(&self) -> bool {
        self.interactive_on
    }

    /// Enters interactive mode
    pub fn enter_interactive(&mut self) {
        self.interactive_on = true;
    }
}

impl Default for ClientConfigState {
    fn default() -> Self {
        let mut state = Self {
            textarea_state: TextAreaState::new(),
            has_focus: false,
            interactive_on: false
        };

        state.load_content();
        state
    }
}

impl Serialize for ClientConfigState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("serializer not implemented")
    }
}
