use std::{io, time::Duration};

use crossterm::event;
use tui::{Terminal, backend::Backend};

use self::{rendering::render, functionality::handle_event, file::File, prompt::Prompt};

mod file;
mod prompt;
mod rendering;
mod functionality;
mod unicode;

#[derive(PartialEq)]
pub enum State { Editing, Saving, Overwriting, Quitting }

pub struct Data {
    state: State,
    file: File,
    save_prompt: Prompt,
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let poll_duration = Duration::from_millis(500);
    let mut data = Data {
        state: State::Editing,
        file: File::new(),
        save_prompt: Prompt::new("Enter file path"),
    };

    while data.state != State::Quitting {
        terminal.draw(|frame| render(frame, &mut data))?;
        if event::poll(poll_duration)? {
            handle_event(event::read()?, &mut data);
        }
    }

    return Ok(());
}