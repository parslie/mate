use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyModifiers, KeyCode, KeyEvent};
use tui::{Terminal, backend::Backend, Frame};

use self::{open_file::OpenFile, file_path::FilePath};

mod open_file;
mod file_path;
mod unicode;

// TODO: create AppState struct to hold all data

fn render<B: Backend>(frame: &mut Frame<B>, open_file: &mut OpenFile, file_path: &mut FilePath, is_saving: bool) {
    open_file.render(frame);
    if is_saving {
        file_path.render(frame);
    }
}

fn edit_events(key: &KeyEvent, open_file: &mut OpenFile, file_path: &mut FilePath, is_saving: &mut bool, is_qutting: &mut bool) {
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        *is_qutting = true;
    } else if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('s') {
        *is_saving = true;
        *file_path = open_file.path.clone();
    }
}

fn save_events(key: &KeyEvent, open_file: &mut OpenFile, file_path: &mut FilePath, is_saving: &mut bool, is_qutting: &mut bool) {
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        *is_saving = false;
    } else if key.code == KeyCode::Enter {
        open_file.path = file_path.clone();
        // TODO: save open file lines to path (create directories if necessary)
    }
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let poll_duration = Duration::from_millis(500);

    let mut is_quitting = false;
    let mut is_saving = false;
    let mut open_file = OpenFile::new();
    let mut file_path = FilePath::new();

    loop {
        terminal.draw(|frame| render(frame, &mut open_file, &mut file_path, is_saving))?;

        if is_quitting {
            return Ok(()); // TODO: warn about unsaved changes
        }

        if event::poll(poll_duration)? {
            if let Event::Key(key) = event::read()? {
                if is_saving {
                    save_events(&key, &mut open_file, &mut file_path, &mut is_saving, &mut is_quitting);
                } else {
                    edit_events(&key, &mut open_file, &mut file_path, &mut is_saving, &mut is_quitting);
                }
            }
        }
    }
}
