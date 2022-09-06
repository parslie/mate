use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyModifiers, KeyCode};
use tui::{Terminal, backend::Backend, Frame, widgets::Paragraph};

use self::unicode::UnicodeString;

pub mod unicode;

fn render<B: Backend>(frame: &mut Frame<B>, text: &UnicodeString) {
    let paragraph = Paragraph::new(text.as_str());
    frame.render_widget(paragraph, frame.size());
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let poll_duration = Duration::from_millis(500);
    let text = UnicodeString::from("This is text.");

    loop {
        terminal.draw(|frame| render(frame, &text))?;

        if event::poll(poll_duration)? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    return Ok(());
                }
            }
        }
    }
}

pub fn test() {
    assert!(UnicodeString::test());
}
