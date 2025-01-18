pub mod color_variants;
pub mod components;
pub mod ui_utils;
pub mod widgets;

use color_eyre::eyre::{Context, Result};
use components::popups::waiting_cmd;
use serde::Serialize;

use crate::{model::Model, terminal::SshyTerminal};

use self::components::{
    popups::{
        add_pub_key, debug_model, error_msg, exit_prompt, prompt_delete_key_pair_confirmation,
        prompt_key_overwrite, set_pub_key_passphrase, with_cfg, Popup,
    },
    sections::{known_hosts_list, public_keys_list, Section},
};

#[derive(Clone, Serialize)]
pub enum Focus {
    Popup(Popup),
    Section(Section),
}

impl Default for Focus {
    fn default() -> Self {
        Self::Section(Section::default())
    }
}

/// Main draw function
///
/// It'll internally draw the appropiate sections and popups based on the given
/// model's state
pub fn draw(terminal: &mut SshyTerminal, model: &Model) -> Result<()> {
    terminal
        .draw(|f| {
            // We first draw the sections
            known_hosts_list::draw(f, model.get_sections_state().get_known_hosts_list_state());
            public_keys_list::draw(f, model.get_sections_state().get_public_keys_list_state());
            // And then the current popup (if any)
            if let Some(popup) = model.get_popup() {
                match popup {
                    Popup::ExitPrompt => exit_prompt::draw_exit_popup(f),
                    Popup::DebugModel => debug_model::draw_debug_model_popup(f, model),
                    Popup::AddPubKey => add_pub_key::draw_add_pub_key_popup(f, model),
                    Popup::WaitingCmd => waiting_cmd::draw_waiting_cmd(f, model),
                    Popup::ErrorMsg => error_msg::draw_error_msg(f, model),
                    Popup::PromptPassphrase => {
                        set_pub_key_passphrase::draw_set_pub_key_passphrase(f, model, false)
                    }
                    Popup::PromptReenterPassphrase => {
                        set_pub_key_passphrase::draw_set_pub_key_passphrase(f, model, true)
                    }
                    Popup::PromptKeyOverwrite => {
                        prompt_key_overwrite::draw_prompt_key_overwrite(f, model)
                    }
                    Popup::WithCfg(content, variant) => {
                        with_cfg::draw_popup_with_cfg(f, content, variant)
                    }
                    Popup::PromptDeleteKeyPairConfirmation => {
                        // Should I change the name? Maybe. Too long.
                        prompt_delete_key_pair_confirmation::draw_prompt_delete_key_pair_confirmation(f, model)
                    },
                }
            }
        })
        .wrap_err("Drawing error")?;

    Ok(())
}
