use crate::ui::{popups::Popup, sections::Section};

#[derive(Clone, Copy)]
pub enum Message {
    StopApp,
    ShowPopup(Popup),
    HidePopup,
    SetSection(Section)
}
