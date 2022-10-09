use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyModifiers, KeyCode};
use tui::{Terminal, backend::Backend, Frame, widgets::{Paragraph, Block, Borders}, layout::Rect};

use self::{unicode::UnicodeString, open_file::OpenFile, file_path::FilePath};

mod unicode;
mod open_file;
mod file_path;

const SAVE_BLOCK_MAX_WIDTH: u16 = 80;

fn render<B: Backend>(frame: &mut Frame<B>, open_file: &OpenFile, is_saving: bool, save_path: &FilePath) {
    let paragraph = Paragraph::new(open_file.to_text(frame.size()));
    frame.render_widget(paragraph, frame.size());

    if is_saving {
        let save_block = Block::default().title(" Enter file path ").borders(Borders::all());
        let save_paragraph = Paragraph::new(save_path.to_spans());

        let mut save_rect = frame.size();
        if save_rect.width > SAVE_BLOCK_MAX_WIDTH {
            save_rect.x = (save_rect.width - SAVE_BLOCK_MAX_WIDTH) / 2;
            save_rect.width = SAVE_BLOCK_MAX_WIDTH;
        }
        save_rect.y = (save_rect.height - 3) / 2;
        save_rect.height = 3;

        frame.render_widget(save_paragraph, save_block.inner(save_rect));
        frame.render_widget(save_block, save_rect);

        let cursor_x = save_path.global_cursor() + save_rect.x + 1;
        frame.set_cursor(cursor_x, save_rect.y + 1);
    } else {
        let (cursor_x, cursor_y) = open_file.clamped_global_cursor();
        frame.set_cursor(cursor_x, cursor_y);
    }
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let poll_duration = Duration::from_millis(500);
    let mut curr_open_file = OpenFile::new();
    let mut is_saving = false;
    let mut save_path = FilePath::new();

    loop {
        terminal.draw(|frame| render(frame, &curr_open_file, is_saving, &save_path))?;

        let area = terminal.size().unwrap_or_else(|_| Rect::new(0, 0, 0, 0));
        let save_area_width = if area.width > SAVE_BLOCK_MAX_WIDTH {
            SAVE_BLOCK_MAX_WIDTH - 2
        } else {
            area.width - 2
        };

        if event::poll(poll_duration)? {
            if let Event::Key(key) = event::read()? {

                if is_saving {
                    if key.code == KeyCode::Enter {
                        // TODO: save open file to file at path (create folders if needed)
                    } else if key.code == KeyCode::Esc {
                        is_saving = false;
                    } 

                    else if key.code == KeyCode::Left {
                        save_path.move_target_left();
                    } else if key.code == KeyCode::Right {
                        save_path.move_target_right(save_area_width);
                    } /*else if key.code == KeyCode::End {
                        curr_open_file.move_target_to_end_of_line(area);
                    } else if key.code == KeyCode::Home {
                        curr_open_file.move_target_to_start_of_line(area);
                    }*/
                    
                    else if key.code == KeyCode::Backspace {
                        save_path.remove_character(true);
                    } else if key.code == KeyCode::Delete {
                        save_path.remove_character(false);
                    }else if let KeyCode::Char(ch) = key.code {
                        save_path.write_character(save_area_width, ch);
                    }
                } else {
                    if key.modifiers == KeyModifiers::CONTROL {
                        if key.code == KeyCode::Char('c') {
                            return Ok(());
                        } else if key.code == KeyCode::Char('s') && !is_saving {
                            is_saving = true;
                            save_path = curr_open_file.path.clone();
                        }
                    } 
                    
                    else if key.code == KeyCode::Up {
                        curr_open_file.move_target_up();
                    } else if key.code == KeyCode::Down {
                        curr_open_file.move_target_down(area);
                    } else if key.code == KeyCode::Left {
                        curr_open_file.move_target_left(area);
                    } else if key.code == KeyCode::Right {
                        curr_open_file.move_target_right(area);
                    } else if key.code == KeyCode::End {
                        curr_open_file.move_target_to_end_of_line(area);
                    } else if key.code == KeyCode::Home {
                        curr_open_file.move_target_to_start_of_line(area);
                    }
    
                    else if key.code == KeyCode::Enter {
                        curr_open_file.break_line(area);
                    } else if key.code == KeyCode::Backspace {
                        curr_open_file.remove_character(area, true);
                    } else if key.code == KeyCode::Delete {
                        curr_open_file.remove_character(area, false);
                    } else if let KeyCode::Char(ch) = key.code {
                        curr_open_file.write_character(area, ch);
                    }
                }
            }
        }
    }
}

#[cfg(debug_assertions)]
pub fn test() {
    assert!(UnicodeString::test());
}
