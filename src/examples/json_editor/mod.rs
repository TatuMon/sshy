use std::{collections::HashMap, error::Error, io};

use color_eyre::eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen},
};
use ratatui::{backend::{CrosstermBackend, Backend}, Terminal};

pub mod ui;

pub fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = JsonEditorApp::new();
    let res = app.run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if res.is_ok() {
        if app.print_json {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

#[derive(Clone)]
pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct JsonEditorApp {
    pub key_input: String,              // the currently being edited json key.
    pub value_input: String,            // the currently being edited json value.
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub print_json: bool, // indicates if the app should print the json on exit
    pub exit: bool, // indicates if the app should exit
}

impl JsonEditorApp {
    fn new() -> Self {
        Self {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            print_json: false,
            exit: false
        }
    }

    pub fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;
            self.handle_input()?;
            if self.exit {
                return Ok(());
            }
        }
    }

    fn handle_input(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                return Ok(());
            }

            match self.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        self.current_screen = CurrentScreen::Exiting;
                    },
                    KeyCode::Char('e') => {
                        self.current_screen = CurrentScreen::Editing;
                        self.currently_editing = Some(CurrentlyEditing::Key);
                    },
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        self.print_json = true;
                        self.exit = true;
                    },
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        self.print_json = false;
                        self.exit = true;
                    },
                    _ => {}
                },
                CurrentScreen::Editing => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        self.currently_editing = Some(CurrentlyEditing::Value);
                                    },
                                    CurrentlyEditing::Value => {
                                        self.save_key_value();
                                        self.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        },
                        KeyCode::Backspace => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        self.key_input.pop();
                                    },
                                    CurrentlyEditing::Value => {
                                        self.value_input.pop();
                                    }
                                }
                            }
                        },
                        KeyCode::Esc => {
                            self.current_screen = CurrentScreen::Main;
                            self.currently_editing = None;
                        },
                        KeyCode::Tab => {
                            self.toggle_editing();
                        },
                        KeyCode::Char(value) => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        self.key_input.push(value);
                                    },
                                    CurrentlyEditing::Value => {
                                        self.value_input.push(value);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    } 

    fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}

impl Default for JsonEditorApp {
    fn default() -> Self {
        Self::new()
    }
}
