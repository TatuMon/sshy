use serde::Serialize;
use tui_textarea::{CursorMove, TextArea};

use crate::{ui::widgets::blocks, utils};

#[derive(Clone)]
pub struct ClientConfigState {
    textarea: TextArea<'static>,
    has_focus: bool,
    /// Indicates if the user entered interactive mode
    interactive_on: bool,
}

impl ClientConfigState {
    // pub fn load_content(&mut self) {
    //     let config_content = utils::files::get_client_config_content().unwrap_or(String::from("FAILED TO LOAD"));
    //     self.textarea_state.set_content(config_content);
    // }

    pub fn get_textarea(&self) -> &TextArea {
        &self.textarea
    }

    pub fn focus(&mut self) {
        self.has_focus = true;
        self.update_textarea_block();
    }

    pub fn unfocus(&mut self) {
        self.has_focus = false;
        self.update_textarea_block();
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
        self.update_textarea_block();
    }

    pub fn quit_interactive(&mut self) {
        self.interactive_on = false;
        self.update_textarea_block();
    }

    pub fn update_textarea_block(&mut self) {
        let block = blocks::ssh_config_block(self.interactive_on, self.has_focus);
        self.textarea.set_block(block);
    }

    pub fn move_cursor(&mut self, cursor_move: CursorMove) {
        self.textarea.move_cursor(cursor_move);
    }
}

impl Default for ClientConfigState {
    fn default() -> Self {
        let config_content =
            utils::files::get_client_config_content().unwrap_or(String::from("FAILED TO LOAD"));
        let mut state = Self {
            textarea: TextArea::from(config_content.lines()),
            has_focus: false,
            interactive_on: false,
        };

        state.update_textarea_block();

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
