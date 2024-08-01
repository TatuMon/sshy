use crate::ui::components::{popups::Popup, sections::Section};


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
