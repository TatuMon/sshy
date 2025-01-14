use crate::{commands, ui::components::popups::Popup};

/// Messages are events that the model has to react to and update it's
/// state
pub enum Message {
    /// Indicates that the app should stop
    StopApp,
    ShowPopup(Popup),
    HidePopup,
    /// Triggers a re-draw of the entire screen
    Draw,
    MoveToNextSection,
    MoveToPrevSection,
    SelNextListItem,
    SelPrevListItem,
    SelNextPopupItem,
    SelPrevPopupItem,
    /// If focused on a text input, it indicates that a char should be written
    WriteChar(char),
    /// If focused on a text input, removes the last character
    PopChar,
    /// If focused on a text input, removes the last word
    PopWord,
    /// Indicates that a new command has been spawned and it must be set as the
    /// currently running one
    CmdSpawned(commands::CmdTask),
    /// Indicates that the currently running command has finished
    CmdFinished,
    /// Indicates that the given String should be printed in the error popup
    PrintError(String),
    /// Indicates that the given String should be printer in the error popup, and when
    FatalError(String),
    /// Prompts the user for input
    PromptNewKeyPassphrase,
    PromptReenterNewKeyPassPhrase,
    CleanNewKeyPassphraseInput,
    ReloadPublicKeysList
}
