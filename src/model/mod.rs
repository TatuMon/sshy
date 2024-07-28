pub mod sections_state;

use serde::Serialize;

use crate::{events::messages::Message, ui::{components::popups::Popup, Focus}};

use self::sections_state::SectionsStates;

#[derive(Default, Clone, PartialEq, Serialize)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

/// Struct in charge of handling the application's state
///
/// Changes made to this struct MUST be issued using the 'update' method, which
/// updates the state based on the given Message
#[derive(Clone, Default, Serialize)]
pub struct Model {
    running_state: RunningState,
    current_popup: Option<Popup>,
    current_focus: Focus,
    previous_focus: Focus,
    sections_states: SectionsStates
}

impl Model {
    pub fn get_popup(&self) -> Option<Popup> {
        self.current_popup
    }

    /// Returns which view is currently the app focusing on
    pub fn get_focus(&self) -> Focus {
        self.current_focus.clone()
    }

    pub fn is_app_done(&self) -> bool {
        self.running_state == RunningState::Done
    }

    /// Updates the model state based on the given message
    pub fn update(&mut self, message: Message) {
        match message {
            Message::StopApp => {
                self.running_state = RunningState::Done;
            }
            Message::ShowPopup(popup) => self.set_popup(Some(popup)),
            Message::HidePopup => self.set_popup(None),
            _ => {},
        }
    }

    /// Returns if there's a popup currently being shown
    pub fn on_popup(&self) -> bool {
        self.current_popup.is_some()
    }

    pub fn get_sections_state(&self) -> &SectionsStates {
        &self.sections_states
    }

    /// Setting a popup should also set the current section
    fn set_popup(&mut self, popup: Option<Popup>) {
        self.current_popup = popup;
        if let Some(popup) = popup {
            self.previous_focus = self.current_focus.clone();
            self.current_focus = Focus::Popup(popup);
        } else {
            self.current_focus = self.previous_focus.clone();
        }
    }
}
