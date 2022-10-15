use crossterm::event::{Event, KeyEvent, KeyModifiers, KeyCode};

use self::save::save;

use super::{Data, State};

mod save;

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
            match save(data, false) {
                Ok(true) => {
                    data.state = State::Editing;
                    data.file.path = data.save_prompt.get_answer().clone();
                },
                Ok(false) => data.state = State::Overwriting,
                Err(error) => panic!("Error on saving file!"), // TODO: handle errors properly
            }
        } 
    }
}

fn handle_overwrite_key(key: KeyEvent, data: &mut Data) {
    if key.modifiers == KeyModifiers::CONTROL {
        if key.code == KeyCode::Char('c') {
            data.state = State::Saving;
        }
    } else {
        if key.code == KeyCode::Left {
            data.overwrite_prompt.move_cursor_left();
        } else if key.code == KeyCode::Right {
            data.overwrite_prompt.move_cursor_right();
        }

        else if key.code == KeyCode::Backspace {
            data.overwrite_prompt.remove_character_before();
        } else if key.code == KeyCode::Delete {
            data.overwrite_prompt.remove_character_after();
        } else if let KeyCode::Char(ch) = key.code {
            data.overwrite_prompt.write_character(ch);
        }

        else if key.code == KeyCode::Enter {
            if data.overwrite_prompt.get_answer().as_str().to_lowercase() == "y" {
                match save(data, true) {
                    Ok(true) => { 
                        data.state = State::Editing;
                        data.file.path = data.save_prompt.get_answer().clone();
                    },
                    Ok(false) => panic!("Did not overwrite file!"), // Should not be possible because of force_overwrite
                    Err(error) => panic!("Error on overwriting file!"), // TODO: handle errors properly
                }
            } else {
                data.state = State::Saving;
            }
        } 
    }
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