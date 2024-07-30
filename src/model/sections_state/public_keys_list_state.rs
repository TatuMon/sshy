use serde::Serialize;

use crate::utils;

type ListItems = Vec<String>;

#[derive(Clone)]
pub struct PublicKeysListState {
    items: ListItems,
    selected_item_idx: Option<usize>,
    has_focus: bool
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
}

impl Default for PublicKeysListState {
    fn default() -> Self {
        let mut state = Self{
            items: vec!(),
            selected_item_idx: None,
            has_focus: false
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
