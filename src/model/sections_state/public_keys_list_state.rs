use color_eyre::{Result, eyre::eyre};
use serde::Serialize;
use std::path::PathBuf;

use crate::{
    commands::ssh_keygen::PublicKeyType,
    utils::{self, files, strings},
};

type ListItems = Vec<String>;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum NewPublicKeyFocus {
    #[default]
    Name,
    Comment,
    // KeyType For now, only ED25519 is available
}
pub struct NewPublicKeyState {
    name: String,
    key_type: PublicKeyType,
    comment: String,
    current_focus: NewPublicKeyFocus,
    passphrase: Option<String>,
    passphrase_check: Option<String>,
}

impl Default for NewPublicKeyState {
    fn default() -> Self {
        let def_keytype = PublicKeyType::default();
        let name: &str = def_keytype.into();
        Self {
            name: format!("id_{}", name),
            key_type: def_keytype,
            comment: String::default(),
            current_focus: NewPublicKeyFocus::Name,
            passphrase: None,
            passphrase_check: None,
        }
    }
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

    pub fn get_passphrase_len(&self) -> usize {
        match &self.passphrase {
            None => 0,
            Some(pass) => pass.len(),
        }
    }

    pub fn get_passphrase_bytes(&self) -> Vec<u8> {
        match &self.passphrase {
            None => vec![],
            Some(pass) => pass.clone().as_bytes().to_vec(),
        }
    }

    pub fn write_name(&mut self, ch: char) {
        self.name.push(ch);
    }

    pub fn del_name_char(&mut self) {
        self.name.pop();
    }

    pub fn del_name_word(&mut self) {
        self.name = strings::del_last_word(self.name.to_owned());
    }

    pub fn write_comment(&mut self, ch: char) {
        self.comment.push(ch);
    }

    pub fn del_comment_char(&mut self) {
        self.comment.pop();
    }

    pub fn del_comment_word(&mut self) {
        self.comment = strings::del_last_word(self.comment.to_owned());
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
            NewPublicKeyFocus::Name => self.current_focus = NewPublicKeyFocus::Comment,
        }
    }

    pub fn write_passphrase(&mut self, ch: char) {
        match &mut self.passphrase {
            None => self.passphrase = Some(String::from(ch)),
            Some(pass) => pass.push(ch),
        }
    }

    pub fn del_passphare_char(&mut self) {
        match &mut self.passphrase {
            Some(pass) => {
                pass.pop();
            }
            _ => {}
        }
    }

    pub fn del_passphrase(&mut self) {
        self.passphrase = None;
    }

    pub fn write_passphrase_check(&mut self, ch: char) {
        match &mut self.passphrase_check {
            None => self.passphrase_check = Some(String::from(ch)),
            Some(pass) => pass.push(ch),
        }
    }

    pub fn del_passphrase_check_char(&mut self) {
        match &mut self.passphrase_check {
            Some(pass) => {
                pass.pop();
            }
            _ => {}
        }
    }

    pub fn del_passphrase_check(&mut self) {
        self.passphrase_check = None;
    }

    pub fn get_passphrase_check_bytes(&self) -> Vec<u8> {
        match &self.passphrase_check {
            None => vec![],
            Some(pass) => pass.clone().as_bytes().to_vec(),
        }
    }

    pub fn get_passphrase_check_len(&self) -> usize {
        match &self.passphrase_check {
            None => 0,
            Some(pass) => pass.len(),
        }
    }

    pub fn clean_passphrases(&mut self) {
        self.passphrase = None;
        self.passphrase_check = None;
    }

    pub fn validate_name(&self) -> Result<(), String> {
        if self.name == "config" {
            return Err(String::from(
                "the name 'config' is reserved to the config file",
            ));
        }

        return Ok(());
    }
}

pub struct PublicKeysListState {
    items: ListItems,
    selected_item_idx: Option<usize>,
    has_focus: bool,
    new_key_state: NewPublicKeyState,
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

    pub fn get_selected_key_name(&self) -> Option<String> {
        let public_key_name = match self.get_selected_item_idx() {
            Some(idx) => self.items.get(idx),
            None => None,
        };

        // I want only the key name, a.k.a the file stem of the public key
        match public_key_name {
            None => None,
            Some(public_key_name) => {
                let pub_key_pathbuf = PathBuf::from(public_key_name);
                pub_key_pathbuf
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
            }
        }
    }

    pub fn get_selected_key_path(&self) -> Option<PathBuf> {
        let public_key_name = match self.get_selected_item_idx() {
            Some(idx) => self.items.get(idx),
            None => None,
        };

        let ssh_dir = files::get_user_ssh_dir().unwrap_or(PathBuf::new());

        public_key_name.map(|n| ssh_dir.join(n))
    }

    pub fn get_selected_key_content(&self) -> Result<String> {
        let public_key_name = match self.get_selected_item_idx() {
            Some(idx) => self.items.get(idx),
            None => None,
        };
        let public_key_name = public_key_name.ok_or(
            eyre!(
                "An error ocurred while trying to read the key name. Use R to refresh the list and try again."
        ))?;

        files::get_pub_key_content(public_key_name)        
    }

    pub fn next_item(&mut self) {
        match self.selected_item_idx {
            None => {
                if !self.items.is_empty() {
                    self.selected_item_idx = Some(0)
                }
            }
            Some(idx) => {
                if idx < self.items.len() - 1 {
                    self.selected_item_idx = Some(idx + 1)
                }
            }
        }
    }

    pub fn prev_item(&mut self) {
        match self.selected_item_idx {
            None => {
                if !self.items.is_empty() {
                    self.selected_item_idx = Some(0)
                }
            }
            Some(idx) => {
                if idx > 0 {
                    self.selected_item_idx = Some(idx - 1)
                }
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
        let mut state = Self {
            items: vec![],
            selected_item_idx: None,
            has_focus: false,
            new_key_state: NewPublicKeyState::default(),
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
        serializer.serialize_str("Serializer not implemented")
    }
}
