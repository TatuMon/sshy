use serde::Serialize;
use tui_textarea::{CursorMove, Input, TextArea};

use crate::{
    model::vim_emulator::{VimMode, VimState},
    ui::widgets::blocks,
    utils::{self, files},
};

#[derive(Clone)]
pub struct ClientConfigState {
    textarea: TextArea<'static>,
    vim_state: VimState,
    has_focus: bool,
    /// Indicates if the user entered interactive mode
    interactive_on: bool,
}

impl ClientConfigState {
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
        let block = blocks::ssh_config_block(self.interactive_on, self.has_focus, &self.vim_state);
        self.textarea.set_block(block);
    }

    pub fn move_cursor(&mut self, cursor_move: CursorMove) {
        self.textarea.move_cursor(cursor_move);
    }

    pub fn set_vim_mode(&mut self, mode: VimMode) {
        if mode == VimMode::Visual {
            self.textarea.start_selection();
        }

        let current_mode = self.vim_state.get_mode();
        if current_mode == VimMode::Visual && mode != VimMode::Visual {
            self.textarea.cancel_selection();
        }

        self.vim_state.set_mode(mode);
        self.update_textarea_block();
    }

    pub fn get_vim_state(&self) -> &VimState {
        &self.vim_state
    }

    pub fn handle_textarea_input(&mut self, input: Input) {
        self.textarea.input(input);
    }

    pub fn textarea_undo(&mut self) {
        self.textarea.undo();
    }

    pub fn textarea_redo(&mut self) {
        self.textarea.redo();
    }

    // If not in visual mode, this method is a no-op
    pub fn textarea_yank(&mut self) {
        if self.vim_state.get_mode() != VimMode::Visual {
            return;
        }

        self.textarea.move_cursor(CursorMove::Forward); // Vim's text selection is inclusive
        self.textarea.copy();
        self.vim_state.set_mode(VimMode::Normal);

        self.update_textarea_block();
    }

    pub fn textarea_paste(&mut self) {
        self.textarea.paste();
    }

    pub fn textarea_write_buffer(&mut self) {
        let content_lines = self.textarea.lines();
        let _ = files::truncate_client_config_content(content_lines).expect("FAILED TO WRITE");
    }
}

impl Default for ClientConfigState {
    fn default() -> Self {
        let config_content =
            utils::files::get_client_config_content().unwrap_or(String::from("FAILED TO LOAD"));
        let mut state = Self {
            textarea: TextArea::from(config_content.lines()),
            vim_state: VimState::default(),
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
