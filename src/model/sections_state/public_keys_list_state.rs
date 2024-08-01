
use serde::Serialize;

use crate::utils::{self, strings};

type ListItems = Vec<String>;

#[derive(Clone, Copy, Default)]
pub enum PublicKeyType {
    #[default]
    ED25519
}

impl From<PublicKeyType> for &str {
    fn from(value: PublicKeyType) -> Self {
        match value {
            PublicKeyType::ED25519 => "ed25519"
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum NewPublicKeyFocus {
    #[default]
    Name,
    Comment,
    // KeyType For now, only ED25519 is available
}

#[derive(Clone, Default)]
pub struct NewPublicKeyState {
    name: String,
    key_type: PublicKeyType,
    comment: String,
    current_focus: NewPublicKeyFocus
}

impl NewPublicKeyState {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_type(&self) -> PublicKeyType {
        self.key_type
    }

    pub fn get_comment(&self) -> &str {
        self.comment.as_str()
    }

    pub fn write_name(&mut self, ch: char) {
        self.name.push(ch);
    }

    pub fn del_name(&mut self) {
        self.name.pop();
    }

    pub fn del_name_word(&mut self) {
        self.name = strings::del_word(self.name.to_owned());
    }

    pub fn write_comment(&mut self, ch: char) {
        self.comment.push(ch);
    }

    pub fn del_comment(&mut self) {
        self.comment.pop();
    }

    pub fn del_comment_word(&mut self) {
        self.comment = strings::del_word(self.comment.to_owned());
    }

    pub fn get_focus(&self) -> NewPublicKeyFocus {
        self.current_focus
    }

    pub fn has_focus_on(&self, possible_focus: NewPublicKeyFocus) -> bool {
        self.current_focus == possible_focus
    }

    pub fn next_focus(&mut self) {
        match self.current_focus {
            NewPublicKeyFocus::Comment => self.current_focus = NewPublicKeyFocus::Name,
            NewPublicKeyFocus::Name => self.current_focus = NewPublicKeyFocus::Comment,
        }
    }

    pub fn prev_focus(&mut self) {
        match self.current_focus {
            NewPublicKeyFocus::Comment => self.current_focus = NewPublicKeyFocus::Name,
            NewPublicKeyFocus::Name => self.current_focus = NewPublicKeyFocus::Comment
        }
    }
}

#[derive(Clone)]
pub struct PublicKeysListState { items: ListItems,
    selected_item_idx: Option<usize>,
    has_focus: bool,
    new_key_state: NewPublicKeyState
}

impl PublicKeysListState {
    pub fn load_public_keys(&mut self) {
        let public_keys = utils::files::get_public_keys_names().unwrap_or_default();
        self.items = public_keys;
    }

    pub fn focus(&mut self) {
        self.has_focus = true;
        if !self.items.is_empty() {
            self.selected_item_idx = Some(0);
        }
    }

    pub fn unfocus(&mut self) {
        self.has_focus = false;
        self.selected_item_idx = None;
    }

    pub fn has_focus(&self) -> bool {
        self.has_focus
    }

    pub fn get_items(&self) -> ListItems {
        self.items.clone()
    }

    pub fn get_selected_item_idx(&self) -> Option<usize> {
        self.selected_item_idx
    }

    pub fn next_item(&mut self) {
        match self.selected_item_idx {
            None => if !self.items.is_empty() {
                self.selected_item_idx = Some(0)
            },
            Some(idx) => if idx < self.items.len()-1 {
                self.selected_item_idx = Some(idx + 1)
            }
        }
    }

    pub fn prev_item(&mut self) {
        match self.selected_item_idx {
            None => if !self.items.is_empty() {
                self.selected_item_idx = Some(0)
            },
            Some(idx) => if idx > 0 {
                self.selected_item_idx = Some(idx - 1)
            }
        }
    }
    
    pub fn get_new_key_state(&self) -> &NewPublicKeyState {
        &self.new_key_state
    }

    pub fn get_new_key_state_mut(&mut self) -> &mut NewPublicKeyState {
        &mut self.new_key_state
    }
}

impl Default for PublicKeysListState {
    fn default() -> Self {
        let mut state = Self{
            items: vec!(),
            selected_item_idx: None,
            has_focus: false,
            new_key_state: NewPublicKeyState::default()
        };

        state.load_public_keys();

        state
    }
}

impl Serialize for PublicKeysListState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("serializer not implemented")
    }
}
