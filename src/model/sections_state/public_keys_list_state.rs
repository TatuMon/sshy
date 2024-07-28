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
        let known_hosts = utils::files::get_public_keys_names().unwrap_or_default();
        self.items = known_hosts;
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

    pub fn get_items(&self) -> ListItems {
        self.items.clone()
    }

    pub fn get_selected_item_idx(&self) -> Option<usize> {
        self.selected_item_idx
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
