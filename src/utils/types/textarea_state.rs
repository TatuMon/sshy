#[derive(Clone)]
pub struct TextAreaState {
    content: String,
    cursor_pos: (u16, u16),
}

impl TextAreaState {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_pos: (0, 0)
        }
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn get_cursor_pos(&self) -> (u16, u16) {
        self.cursor_pos
    }

    pub fn set_cursor_pos(&mut self, pos: (u16, u16)) {
        self.cursor_pos = pos;
    }
}
