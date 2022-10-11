use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyModifiers, KeyCode, KeyEvent};
use tui::{Terminal, backend::Backend, Frame};

use self::{open_file::OpenFile, file_path::FilePath};

mod open_file;
mod file_path;
mod unicode;

struct AppState {
    open_file: OpenFile,
    file_path: FilePath,
    is_saving: bool,
    is_quitting: bool,
}

fn render<B: Backend>(frame: &mut Frame<B>, app_state: &mut AppState) {
    app_state.open_file.render(frame);
    if app_state.is_saving {
        app_state.file_path.render(frame);
    }
}

fn edit_events(key: &KeyEvent, app_state: &mut AppState) {
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        app_state.is_quitting = true;
    } else if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('s') {
        app_state.is_saving = true;
        app_state.file_path = app_state.open_file.path.clone();
    } 
    
    else if key.code == KeyCode::Backspace {
        app_state.open_file.remove_character_before();
    } else if key.code == KeyCode::Delete {
        app_state.open_file.remove_character_after();
    } else if key.code == KeyCode::Enter {
        app_state.open_file.break_line();
    } else if let KeyCode::Char(ch) = key.code {
        app_state.open_file.write_character(ch);
    }

    else if key.code == KeyCode::Up {
        app_state.open_file.move_cursor_up();
    } else if key.code == KeyCode::Down {
        app_state.open_file.move_cursor_down();
    } else if key.code == KeyCode::Left {
        app_state.open_file.move_cursor_left();
    } else if key.code == KeyCode::Right {
        app_state.open_file.move_cursor_right();
    }
}

fn save_events(key: &KeyEvent, app_state: &mut AppState) {
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        app_state.is_saving = false;
    } else if key.code == KeyCode::Enter {
        app_state.is_saving = false;
        app_state.open_file.path = app_state.file_path.clone();
        // TODO: save open file lines to path (create directories if necessary)
    }

    else if key.code == KeyCode::Backspace {
        app_state.file_path.remove_character_before();
    } else if key.code == KeyCode::Delete {
        app_state.file_path.remove_character_after();
    } else if let KeyCode::Char(ch) = key.code {
        app_state.file_path.write_character(ch);
    }

    else if key.code == KeyCode::Left {
        app_state.file_path.move_cursor_left();
    } else if key.code == KeyCode::Right {
        app_state.file_path.move_cursor_right();
    }
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let poll_duration = Duration::from_millis(500);
    let mut app_state = AppState {
        open_file: OpenFile::new(),
        file_path: FilePath::new(),
        is_saving: false,
        is_quitting: false,
    };

    loop {
        terminal.draw(|frame| render(frame, &mut app_state))?;

        if app_state.is_quitting {
            return Ok(()); // TODO: warn about unsaved changes
        }

        if event::poll(poll_duration)? {
            if let Event::Key(key) = event::read()? {
                if app_state.is_saving {
                    save_events(&key, &mut app_state);
                } else {
                    edit_events(&key, &mut app_state);
                }
            }
        }
    }
}
