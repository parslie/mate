use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyModifiers, KeyCode};
use tui::{Terminal, backend::Backend, Frame, widgets::Paragraph};

use self::{unicode::UnicodeString, open_file::OpenFile};

mod unicode;
mod open_file;

fn render<B: Backend>(frame: &mut Frame<B>, open_file: &OpenFile) {
    let paragraph = Paragraph::new(open_file.to_text());
    frame.render_widget(paragraph, frame.size());

    // TODO: implement proper cursor pos after implementing viewport
    let (cursor_x, cursor_y) = open_file.target_pos();
    frame.set_cursor(cursor_x as u16, cursor_y as u16);
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let poll_duration = Duration::from_millis(500);

    let mut curr_open_file = OpenFile::new();

    loop {
        terminal.draw(|frame| render(frame, &curr_open_file))?;

        if event::poll(poll_duration)? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    return Ok(());
                }

                if key.code == KeyCode::Up {
                    curr_open_file.move_target_up();
                } else if key.code == KeyCode::Down {
                    curr_open_file.move_target_down();
                } else if key.code == KeyCode::Left {
                    curr_open_file.move_target_left();
                } else if key.code == KeyCode::Right {
                    curr_open_file.move_target_right();
                }

                else if key.code == KeyCode::Enter {
                    curr_open_file.break_line();
                } else if key.code == KeyCode::Backspace {
                    curr_open_file.remove_character(true);
                } else if key.code == KeyCode::Delete {
                    curr_open_file.remove_character(false);
                } else if let KeyCode::Char(ch) = key.code {
                    curr_open_file.write_character(ch);
                }
            }
        }
    }
}

#[cfg(debug_assertions)]
pub fn test() {
    assert!(UnicodeString::test());
}
