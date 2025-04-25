use crate::{commands, ui::components::popups::Popup, model::vim_emulator::VimMode};
use tui_textarea::{CursorMove, Scrolling, Input};

/// Messages are events that the model has to react to and update it's
/// state
#[derive(Debug)]
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
    /// Indicates that the given String should be printed in the success popup
    PrintSuccess(String),
    /// Indicates that the given String should be printed in the error popup
    PrintError(String),
    /// Indicates that the given String should be printer in the error popup, and when
    FatalError(String),
    /// Prompts the user for input
    PromptNewKeyPassphrase,
    PromptReenterNewKeyPassPhrase,
    PromptKeyOverwrite,
    CleanNewKeyPassphraseInput,
    RefreshPublicKeysList,
    RefreshKnownHostsList,
    PromptDeleteKeyPairConfirmation,
    /// Enters interactive mode with the focused textarea, initiating a Vim state machine and
    /// updating the textarea's section state
    TextAreaInteract,
    /// Delete from cursor to line end and enter vim normal mode
    TextAreaDeleteToEnd,
    /// Delete selected text and enter vim normal mode
    TextAreaDelete,
    /// Delete line and enter vim normal mode
    TextAreaDeleteLine,
    /// Delete selected text and enter vim insert mode
    TextAreaCut,
    /// Delete from cursor to line end and enter vim insert mode
    TextAreaCutToEnd,
    /// Delete line and enter vim insert mode
    TextAreaCutLine,
    /// Start selection and enter vim visual mode
    TextAreaStartSelection,
    /// Start line selection and enter vim visual mode
    TextAreaStartLineSelection,
    /// Cancel selection and enter vim normal mode
    TextAreaCancelSelection,
    /// Paste yanked text and enter vim normal mode
    TextAreaPaste,
    /// Undo last change and enter vim normal mode
    TextAreaUndo,
    /// Redo last change and enter vim normal mode
    TextAreaRedo,
    /// Delete next char and enter vim normal mode
    TextAreaDeleteNextChar,
    /// Enters vim insert mode, creating a new line after the current one
    TextAreaInsertAtNewlineAfter,
    /// Enters vim insert mode, creating a new line before the current one
    TextAreaInsertAtNewlineBefore,
    /// Enters vim insert mode, positioning the cursor after the char
    TextAreaInsertAfter,
    /// Enters vim insert mode, positioning the cursor at the end of the line
    TextAreaInsertAtEnd,
    /// Enters vim insert mode, positioning the cursor at the start of the line
    TextAreaInsertAtStart,
    TextAreaMoveCursor(CursorMove),
    TextAreaScroll(Scrolling),
    // Yank and enter vim normal mode
    TextAreaYank,
    TextAreaInput(Input),
    SetVimMode(VimMode),
    SetVimPendingInput(Input),
    VimQuit,
    // Confirm buffer writing to file
    TextAreaWriteBuffer,
    // FALTAN
}
