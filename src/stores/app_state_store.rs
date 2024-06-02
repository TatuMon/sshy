use color_eyre::eyre::Result;

use crate::actions::AppStateAction;

pub enum CurrentScreen {
    Main,
    Exiting
}

pub struct AppStateStore {
    current_screen: CurrentScreen,
    running: bool
}

impl AppStateStore {
    pub fn default() -> Self {
        Self {
            current_screen: CurrentScreen::Main,
            running: true
        }
    }

    pub fn update(&mut self, action: AppStateAction) -> Result<()> {
        match action {
            AppStateAction::Exit => {
                self.current_screen = CurrentScreen::Exiting
            },
            AppStateAction::StopApp => {
                self.running = false
            },
            _ => todo!()
        }

        Ok(())
    }
}
