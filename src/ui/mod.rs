pub mod color_variants;
pub mod components;
pub mod ui_utils;
pub mod widgets;
pub mod app_layout;

use color_eyre::eyre::{Context, Result};
use components::{footers, popups::waiting_cmd};
use app_layout::AppLayout;

use crate::{model::Model, terminal::SshyTerminal};

use self::components::{
    popups::{
        add_pub_key, error_msg, exit_prompt, prompt_delete_key_pair_confirmation,
        prompt_key_overwrite, set_pub_key_passphrase, with_cfg, show_pub_key_content, Popup,
    },
    sections::{known_hosts_list, public_keys_list, client_config, Section},
};

#[derive(Clone)]
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
            let app_layout = AppLayout::from_frame(f);
            footers::main_footer::draw_footer(f, &app_layout.main_footer, model);
            known_hosts_list::draw(f, &app_layout.known_hosts_list, model.get_sections_state().get_known_hosts_list_state());
            public_keys_list::draw(f, &app_layout.public_keys_list, model.get_sections_state().get_public_keys_list_state());
            client_config::draw(f, &app_layout.client_config, model.get_sections_state().get_client_config_state());
            if let Some(popup) = model.get_popup() {
                match popup {
                    Popup::ExitPrompt => exit_prompt::draw_exit_popup(f),
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
                    Popup::ShowPubKeyContent => {
                        show_pub_key_content::draw_pub_key_content(f, model)
                    }
                }
            }
        })
        .wrap_err("Drawing error")?;

    Ok(())
}
