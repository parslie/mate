use tui::{Frame, backend::Backend, layout::{Layout, Constraint, Direction}, widgets::{Block, Borders}};

use super::{Data, State};

pub fn render<B: Backend>(frame: &mut Frame<B>, data: &mut Data) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(frame.size());

    let file_block =  Block::default().borders(Borders::all());
    let file_rect = file_block.inner(layout[0]);
    data.file.adjust_viewport(file_rect);
    data.file.render(frame, file_rect);
    frame.render_widget(file_block, layout[0]);

    match data.state {
        State::Editing => {
            let cursor = data.file.global_cursor(file_rect);
            frame.set_cursor(cursor.0, cursor.1);
        },
        State::Saving => {
            let mut prompt_rect = layout[1];
            prompt_rect.width -= 2;
            prompt_rect.x += 1;
            data.save_prompt.adjust_viewport(prompt_rect);
            data.save_prompt.render(frame, prompt_rect);
            let cursor = data.save_prompt.global_cursor(prompt_rect);
            frame.set_cursor(cursor.0, cursor.1);
        },
        _ => (),
    }
}