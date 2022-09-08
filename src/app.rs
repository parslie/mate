use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyModifiers, KeyCode};
use tui::{Terminal, backend::Backend, Frame, widgets::Paragraph, text::{Text, Spans}};

use self::unicode::UnicodeString;

pub mod unicode;

fn render<B: Backend>(frame: &mut Frame<B>, lines: &Vec<UnicodeString>) {
    let mut lines_spans = Vec::new();
    for line in lines {
        lines_spans.push(Spans::from(line.as_str()));
    }
    let text = Text::from(lines_spans);
    
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, frame.size());
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let poll_duration = Duration::from_millis(500);

    let mut lines = vec![UnicodeString::from("öTesting"), UnicodeString::from("New line")];
    let mut target_line: usize = 0;
    let mut target_char: usize = 0;

    loop {
        terminal.draw(|frame| render(frame, &lines))?;

        if event::poll(poll_duration)? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    return Ok(());
                }

                let lines_len = lines.len();
                let curr_line = lines.get_mut(target_line)
                    .expect("should never index a line out-of-bounds");

                if key.code == KeyCode::Enter {
                    // TODO: implement UnicodeString::drain first
                } else if key.code == KeyCode::Backspace {
                    target_char = target_char.clamp(0, curr_line.length());
                    if target_char > 0 {
                        curr_line.remove(target_char - 1);
                        target_char -= 1;
                    } else if target_line > 0 {
                        let curr_line = lines.remove(target_line);
                        let prev_line = lines.get_mut(target_line - 1)
                            .expect("should never index a line out-of-bounds");
                        prev_line.push_str(curr_line.as_str());
                        target_line -= 1;
                        target_char = prev_line.length();
                    }
                } else if key.code == KeyCode::Delete {
                    target_char = target_char.clamp(0, curr_line.length());
                    if target_char < curr_line.length() {
                        curr_line.remove(target_line);
                    } else if target_line < lines_len - 1 {
                        let next_line = lines.remove(target_line + 1);
                        let curr_line = lines.get_mut(target_line)
                            .expect("should never index a line out-of-bounds");
                        curr_line.push_str(next_line.as_str());
                    }
                } else if let KeyCode::Char(ch) = key.code {
                    target_char = target_char.clamp(0, curr_line.length());
                    curr_line.insert(target_char, ch);
                    target_char += 1;
                }
            }
        }
    }
}

#[cfg(debug_assertions)]
pub fn test() {
    assert!(UnicodeString::test());
}
