use std::{io, time::Duration};

use crossterm::event;
use tui::{Terminal, backend::Backend};

use self::{rendering::render, functionality::handle_event};

mod rendering;
mod functionality;

#[derive(PartialEq)]
enum State { Editing, Saving, Overwriting, Quitting }

struct Data {
    state: State,
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let poll_duration = Duration::from_millis(500);
    let data = Data {
        state: State::Editing,
    };

    while data.state != State::Quitting {
        terminal.draw(|frame| render(frame))?;
        if event::poll(poll_duration)? {
            handle_event(event::read()?);
        }
    }

    return Ok(());
}