use ratatui::widgets::ListState;
use serde::Serialize;

#[derive(Clone, Default)]
pub struct PublicKeysListState {
    items: Vec<String>,
    list_state: ListState
}

impl Serialize for PublicKeysListState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("serializer not implemented")
    }
}
