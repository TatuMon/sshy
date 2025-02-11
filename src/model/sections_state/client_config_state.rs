use std::borrow::Cow;
use serde::Serialize;

use crate::utils;

type ListItems = Vec<String>;

#[derive(Clone)]
pub struct ClientConfigState {
    content: Option<String>,
    has_focus: bool,
    /// Indicates if the user entered "modifying" state
    is_modifying: bool
}

impl ClientConfigState {
    /// Returns the content. Being an empty string in case self.content is None
    pub fn get_content(&self) -> Cow<String> {
        match &self.content {
            None => Cow::Owned(String::new()),
            Some(content) => Cow::Borrowed(content)
        }
    }

    pub fn load_content(&mut self) {
        let config_content = utils::files::get_client_config_content().unwrap_or(String::from("FAILED TO LOAD"));
        self.content = Some(config_content);
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

    pub fn is_user_modifying(&self) -> bool {
        self.is_modifying
    }
}

impl Default for ClientConfigState {
    fn default() -> Self {
        let mut state = Self {
            content: None,
            has_focus: false,
            is_modifying: false
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
