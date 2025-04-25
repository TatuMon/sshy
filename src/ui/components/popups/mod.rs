pub mod add_pub_key;
pub mod exit_prompt;
pub mod waiting_cmd;
pub mod error_msg;
pub mod set_pub_key_passphrase;
pub mod with_cfg;
pub mod prompt_key_overwrite;
pub mod prompt_delete_key_pair_confirmation;
pub mod show_pub_key_content;

use crate::ui::color_variants::ColorVariant;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Popup {
    ExitPrompt,
    AddPubKey,
    WaitingCmd,
    ErrorMsg,
    PromptPassphrase,
    PromptReenterPassphrase,
    PromptKeyOverwrite,
    WithCfg(String, ColorVariant),
    PromptDeleteKeyPairConfirmation,
    ShowPubKeyContent
}
