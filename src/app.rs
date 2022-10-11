use std::{io::{self, Write}, time::Duration, fs::{self, File}, path::Path};

use crossterm::event::{self, Event, KeyModifiers, KeyCode, KeyEvent};
use tui::{Terminal, backend::Backend, Frame};

use self::{open_file::OpenFile, file_path::FilePath, prompt::render_prompt};

mod open_file;
mod file_path;
mod prompt;
mod unicode;

enum AppState { Editing, Saving, Overwriting, Quitting }

struct AppData {
    open_file: OpenFile,
    file_path: FilePath,
    state: AppState,
}

// TODO: consider separating rendering & data/functionality

fn render<B: Backend>(frame: &mut Frame<B>, app_data: &mut AppData) {
    match app_data.state {
        AppState::Overwriting => {
            let file_name = app_data.file_path.as_str().split('/').last().expect("should not be empty while overwriting");

            app_data.open_file.render(frame);
            render_prompt(frame, " Warning! ", format!("Are you sure you want to overwrite '{}'?", file_name).as_str());
        },
        AppState::Saving => {
            app_data.open_file.render(frame);
            app_data.file_path.render(frame);
        },
        _ => app_data.open_file.render(frame),
    }
}

fn edit_events(key: &KeyEvent, app_data: &mut AppData) {
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        app_data.state = AppState::Quitting;
    } else if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('s') {
        app_data.state = AppState::Saving;
        app_data.file_path = app_data.open_file.path.clone();
    } 
    
    else if key.code == KeyCode::Backspace {
        app_data.open_file.remove_character_before();
    } else if key.code == KeyCode::Delete {
        app_data.open_file.remove_character_after();
    } else if key.code == KeyCode::Enter {
        app_data.open_file.break_line();
    } else if let KeyCode::Char(ch) = key.code {
        app_data.open_file.write_character(ch);
    }

    else if key.code == KeyCode::Up {
        app_data.open_file.move_cursor_up();
    } else if key.code == KeyCode::Down {
        app_data.open_file.move_cursor_down();
    } else if key.code == KeyCode::Left {
        app_data.open_file.move_cursor_left();
    } else if key.code == KeyCode::Right {
        app_data.open_file.move_cursor_right();
    }
}

fn save_events(key: &KeyEvent, app_data: &mut AppData) {
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        app_data.state = AppState::Editing;
    } else if key.code == KeyCode::Enter {
        if let Err(_error) = save(app_data, false) {
            // TODO: handle errors
        }
    }

    else if key.code == KeyCode::Backspace {
        app_data.file_path.remove_character_before();
    } else if key.code == KeyCode::Delete {
        app_data.file_path.remove_character_after();
    } else if let KeyCode::Char(ch) = key.code {
        app_data.file_path.write_character(ch);
    }

    else if key.code == KeyCode::Left {
        app_data.file_path.move_cursor_left();
    } else if key.code == KeyCode::Right {
        app_data.file_path.move_cursor_right();
    }
}

fn overwrite_events(key: &KeyEvent, app_data: &mut AppData) {
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        app_data.state = AppState::Saving;
    } else if key.code == KeyCode::Enter {
        if let Err(_error) = save(app_data, true) {
            // TODO: handle errors
        }
    }
}

fn save(app_data: &mut AppData, force_overwrite: bool) -> Result<(), io::Error> {
    let file_exists = Path::new(app_data.file_path.as_str()).exists();

    if file_exists && force_overwrite || !file_exists {
        let mut file = File::create(app_data.file_path.as_str())?;
        file.write_all(app_data.open_file.to_string().as_bytes())?;
        app_data.open_file.path = app_data.file_path.clone();
        app_data.state = AppState::Editing;
    } else if file_exists {
        app_data.state = AppState::Overwriting;
    }

    return Ok(());
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let poll_duration = Duration::from_millis(500);
    let mut app_data = AppData {
        open_file: OpenFile::new(),
        file_path: FilePath::new(),
        state: AppState::Editing,
    };

    loop {
        terminal.draw(|frame| render(frame, &mut app_data))?;

        if let AppState::Quitting = app_data.state {
            return Ok(()); // TODO: warn about unsaved changes
        }

        if event::poll(poll_duration)? {
            if let Event::Key(key) = event::read()? {
                match app_data.state {
                    AppState::Overwriting => overwrite_events(&key, &mut app_data),
                    AppState::Saving => save_events(&key, &mut app_data),
                    _ => edit_events(&key, &mut app_data),
                }
            }
        }
    }
}
