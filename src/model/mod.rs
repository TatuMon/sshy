use serde::Serialize;

use crate::{events::messages::Message, ui::{popups::Popup, sections::Section, Focus}};

#[derive(Default, Clone, PartialEq, Serialize)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Clone, Default, Serialize)]
pub struct Model {
    running_state: RunningState,
    current_popup: Option<Popup>,
    current_section: Section,
    current_focus: Focus,
    previous_section: Section,
}

impl Model {
    pub fn get_section(&self) -> Section {
        self.current_section
    }

    pub fn get_popup(&self) -> Option<Popup> {
        self.current_popup
    }

    pub fn get_focus(&self) -> Focus {
        self.current_focus
    }

    pub fn is_app_done(&self) -> bool {
        self.running_state == RunningState::Done
    }

    /// Apply changes based on the given message
    pub fn update(&mut self, message: Message) {
        match message {
            Message::StopApp => {
                self.running_state = RunningState::Done;
            }
            Message::ShowPopup(popup) => self.set_popup(Some(popup)),
            Message::HidePopup => self.set_popup(None),
            Message::SetSection(section) => self.set_section(section)
        }
    }

    /// Returns if there's a popup currently being shown
    pub fn on_popup(&self) -> bool {
        self.current_popup.is_some()
    }

    fn set_section(&mut self, section: Section) {
        self.previous_section = self.current_section;
        self.current_section = section;
        self.current_focus = Focus::Section(section)
    }

    /// Setting a popup should also set the current section
    fn set_popup(&mut self, popup: Option<Popup>) {
        self.current_popup = popup;
        if let Some(popup) = popup {
            self.current_focus = Focus::Popup(popup);
        } else {
            self.current_focus = Focus::Section(self.current_section);
        }
    }
}
