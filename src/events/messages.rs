use crate::ui::components::{popups::Popup, sections::Section};

/// Messages are events that the model has to react to and update it's
/// state
#[derive(Clone)]
pub enum Message {
    StopApp,
    ShowPopup(Popup),
    HidePopup,
    SetSection(Section),
    Draw,
    MoveToNextSection,
    MoveToPrevSection,
    SelNextListItem,
    SelPrevListItem,
    SelNextPopupItem,
    SelPrevPopupItem,
    WriteChar(char),
    PopChar,
    PopWord
}
