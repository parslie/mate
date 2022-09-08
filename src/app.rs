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

    let mut lines = vec![UnicodeString::new()];
    let mut target_line: usize = 0;
    let mut target_char: usize = 0;

    loop {
        terminal.draw(|frame| render(frame, &lines))?;

        if event::poll(poll_duration)? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    return Ok(());
                }

                let curr_line = lines.get_mut(target_line)
                    .expect("should never index a line out-of-bounds");

                if key.code == KeyCode::Enter {
                    // TODO: implement UnicodeString::drain first
                } else if key.code == KeyCode::Backspace {
                    // TODO: implement UnicodeString::push_str first
                } else if key.code == KeyCode::Delete {
                    // TODO: implement UnicodeString::push_str first
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
