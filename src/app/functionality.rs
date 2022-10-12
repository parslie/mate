use crossterm::event::{Event, KeyEvent, KeyModifiers, KeyCode};

use super::{Data, State};

fn handle_edit_key(key: KeyEvent, data: &mut Data) {
    if key.modifiers == KeyModifiers::CONTROL {
        if key.code == KeyCode::Char('c') {
            data.state = State::Quitting;
        } else if key.code == KeyCode::Char('s') {
            data.state = State::Saving;
            data.save_prompt.set_answer(&data.file.path);
        }
    } else {
        if key.code == KeyCode::Up {
            data.file.move_cursor_up();
        } else if key.code == KeyCode::Down {
            data.file.move_cursor_down();
        } else if key.code == KeyCode::Left {
            data.file.move_cursor_left();
        } else if key.code == KeyCode::Right {
            data.file.move_cursor_right();
        }

        else if key.code == KeyCode::Backspace {
            data.file.remove_character_before();
        } else if key.code == KeyCode::Delete {
            data.file.remove_character_after();
        } else if key.code == KeyCode::Enter {
            data.file.break_line();
        } else if let KeyCode::Char(ch) = key.code {
            data.file.write_character(ch);
        }
    }
}

fn handle_save_key(key: KeyEvent, data: &mut Data) {
    if key.modifiers == KeyModifiers::CONTROL {
        if key.code == KeyCode::Char('c') {
            data.state = State::Editing;
        }
    } else {
        if key.code == KeyCode::Left {
            data.save_prompt.move_cursor_left();
        } else if key.code == KeyCode::Right {
            data.save_prompt.move_cursor_right();
        }

        else if key.code == KeyCode::Backspace {
            data.save_prompt.remove_character_before();
        } else if key.code == KeyCode::Delete {
            data.save_prompt.remove_character_after();
        } else if let KeyCode::Char(ch) = key.code {
            data.save_prompt.write_character(ch);
        }

        else if key.code == KeyCode::Enter {
            // TODO: save functionality
        } 
    }
}

fn handle_overwrite_key(_key: KeyEvent, _data: &mut Data) {

}

pub fn handle_event(event: Event, data: &mut Data) {
    if let Event::Key(key) = event {
        match data.state {
            State::Overwriting => handle_overwrite_key(key, data),
            State::Saving => handle_save_key(key, data),
            _ => handle_edit_key(key, data),
        }
    }
}