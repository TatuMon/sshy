use crate::events::messages::Message;
use crate::model::vim_emulator::{VimMode, VimState};
use tui_textarea::{CursorMove, Input, Key, Scrolling, TextArea};

// How the Vim emulation state transitions
pub enum VimTransition {
    Nop,
    Mode(VimMode),
    Pending(Input),
    Quit,
}

fn handle_transition(transition: VimTransition) -> Message {
    match transition {
        VimTransition::Mode(mode) => Message::SetVimMode(mode),
        VimTransition::Pending(input) => Message::SetVimPendingInput(input),
        VimTransition::Quit => Message::VimQuit,
        VimTransition::Nop => Message::Draw,
    }
}

/// Handles the given input based on the given vim mode
///
/// # Arguments
///
/// * `input` - The input being handled
/// * `vim_state` - The handling is based on this given vim state
/// * `textarea_state` - The handling is also based on this given textarea state
pub fn handle_key_input(input: Input, vim_state: &VimState) -> Message {
    if input.key == Key::Null {
        return Message::Draw;
    }

    let vim_mode = vim_state.get_mode();

    return match vim_mode {
        VimMode::Normal | VimMode::Visual | VimMode::Operator(_) => {
            match input {
                Input {
                    key: Key::Char('h'),
                    ..
                } => Message::TextAreaMoveCursor(CursorMove::Back),
                Input {
                    key: Key::Char('j'),
                    ..
                } => Message::TextAreaMoveCursor(CursorMove::Down),
                Input {
                    key: Key::Char('k'),
                    ..
                } => Message::TextAreaMoveCursor(CursorMove::Up),
                Input {
                    key: Key::Char('l'),
                    ..
                } => Message::TextAreaMoveCursor(CursorMove::Forward),
                Input {
                    key: Key::Char('w'),
                    ..
                } => Message::TextAreaMoveCursor(CursorMove::WordForward),
                Input {
                    key: Key::Char('e'),
                    ctrl: false,
                    ..
                } => Message::TextAreaMoveCursor(CursorMove::WordEnd),
                Input {
                    key: Key::Char('b'),
                    ctrl: false,
                    ..
                } => Message::TextAreaMoveCursor(CursorMove::WordBack),
                Input {
                    key: Key::Char('^'),
                    ..
                } => Message::TextAreaMoveCursor(CursorMove::Head),
                Input {
                    key: Key::Char('$'),
                    ..
                } => Message::TextAreaMoveCursor(CursorMove::End),
                Input {
                    key: Key::Char('D'),
                    ..
                } => Message::TextAreaDeleteToEnd,
                Input {
                    key: Key::Char('C'),
                    ..
                } => Message::TextAreaCutToEnd,
                Input {
                    key: Key::Char('p'),
                    ..
                } => Message::TextAreaPaste,
                Input {
                    key: Key::Char('u'),
                    ctrl: false,
                    ..
                } => Message::TextAreaUndo,
                Input {
                    key: Key::Char('r'),
                    ctrl: true,
                    ..
                } => Message::TextAreaRedo,
                Input {
                    key: Key::Char('x'),
                    ..
                } => Message::TextAreaDeleteNextChar,
                Input {
                    key: Key::Char('i'),
                    ..
                } => Message::SetVimMode(VimMode::Insert),
                Input {
                    key: Key::Char('a'),
                    ..
                } => Message::TextAreaInsertAfter,
                Input {
                    key: Key::Char('A'),
                    ..
                } => Message::TextAreaInsertAtEnd,
                Input {
                    key: Key::Char('o'),
                    ..
                } => Message::TextAreaInsertAtNewlineAfter,
                Input {
                    key: Key::Char('O'),
                    ..
                } => Message::TextAreaInsertAtNewlineBefore,
                Input {
                    key: Key::Char('I'),
                    ..
                } => Message::TextAreaInsertAtStart,
                Input {
                    key: Key::Char('q'),
                    ..
                } => Message::VimQuit,
                Input {
                    key: Key::Char('e'),
                    ctrl: true,
                    ..
                } => Message::TextAreaScroll((1, 0).into()),
                Input {
                    key: Key::Char('y'),
                    ctrl: true,
                    ..
                } => Message::TextAreaScroll((-1, 0).into()),
                Input {
                    key: Key::Char('d'),
                    ctrl: true,
                    ..
                } => Message::TextAreaScroll(Scrolling::HalfPageDown),
                Input {
                    key: Key::Char('u'),
                    ctrl: true,
                    ..
                } => Message::TextAreaScroll(Scrolling::HalfPageUp),
                Input {
                    key: Key::Char('f'),
                    ctrl: true,
                    ..
                } => Message::TextAreaScroll(Scrolling::PageDown),
                Input {
                    key: Key::Char('b'),
                    ctrl: true,
                    ..
                } => Message::TextAreaScroll(Scrolling::PageUp),
                Input {
                    key: Key::Char('v'),
                    ctrl: false,
                    ..
                } => Message::TextAreaStartSelection,
                Input {
                    key: Key::Char('V'),
                    ctrl: false,
                    ..
                } if vim_mode == VimMode::Normal => Message::TextAreaStartLineSelection,
                Input { key: Key::Esc, .. }
                | Input {
                    key: Key::Char('v'),
                    ctrl: false,
                    ..
                } if vim_mode == VimMode::Visual => Message::TextAreaCancelSelection,
                Input {
                    key: Key::Char('g'),
                    ctrl: false,
                    ..
                } if matches!(
                    vim_state.get_pending_input(),
                    Input {
                        key: Key::Char('g'),
                        ctrl: false,
                        ..
                    }
                ) => Message::TextAreaMoveCursor(CursorMove::Top),
                Input {
                    key: Key::Char('G'),
                    ctrl: false,
                    ..
                } => Message::TextAreaMoveCursor(CursorMove::Bottom),
                // TODO
                // See how to translate this to a single Message
                //
                // Input {
                //     key: Key::Char(c),
                //     ctrl: false,
                //     ..
                // } if vim_mode == VimMode::Operator(c) => {
                // Handle yy, dd, cc. (This is not strictly the same behavior as Vim)
                // textarea.move_cursor(CursorMove::Head);
                // textarea.start_selection();
                // let cursor = textarea.cursor();
                // textarea.move_cursor(CursorMove::Down);
                // if cursor == textarea.cursor() {
                //     textarea.move_cursor(CursorMove::End); // At the last line, move to end of the line instead
                // }
                // }
                //
                // TODO
                // See how to translate this to a single Message
                // Input {
                //     key: Key::Char(op @ ('y' | 'd' | 'c')),
                //     ctrl: false,
                //     ..
                // } if vim_mode == VimMode::Normal => vec![
                //     Message::TextAreaStartSelection,
                //     Message::SetVimMode(VimMode::Operator(op)),
                // ],
                Input {
                    key: Key::Char('y'),
                    ctrl: false,
                    ..
                } if vim_mode == VimMode::Visual => Message::TextAreaYank,
                    // textarea.move_cursor(CursorMove::Forward); // Vim's text selection is inclusive
                    // textarea.copy();
                    // return VimTransition::Mode(VimMode::Normal);
                Input {
                    key: Key::Char('d'),
                    ctrl: false,
                    ..
                } if vim_mode == VimMode::Visual => Message::TextAreaDelete,
                    // textarea.move_cursor(CursorMove::Forward); // Vim's text selection is inclusive
                    // textarea.cut();
                    // return VimTransition::Mode(VimMode::Normal);
                Input {
                    key: Key::Char('c'),
                    ctrl: false,
                    ..
                } if vim_mode == VimMode::Visual => Message::TextAreaCut,
                    // textarea.move_cursor(CursorMove::Forward); // Vim's text selection is inclusive
                    // textarea.cut();
                    // return VimTransition::Mode(VimMode::Insert);
                input => Message::SetVimPendingInput(input),
            }
        }
        VimMode::Insert => match input {
            Input { key: Key::Esc, .. }
            | Input {
                key: Key::Char('c'),
                ctrl: true,
                ..
            } => Message::SetVimMode(VimMode::Normal),
            input => {
                Message::TextAreaInput(input) // Use default key mappings in insert mode
            }
        },
    }
}

pub fn handle_vim_operator(operator: VimMode) -> Message {
    // Handle the pending operator
    match operator {
        VimMode::Operator('y') => {
            Message::TextAreaYank
        }
        VimMode::Operator('d') => {
            Message::TextAreaDelete
        }
        VimMode::Operator('c') => {
            Message::TextAreaCut
        }
        _ => Message::Draw,
    }
}
